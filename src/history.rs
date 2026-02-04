use crate::config::projects_dir;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Cost and session information from JSONL history
#[derive(Debug, Default)]
pub struct HistoryData {
    pub daily_cost: f64,
    pub session_start: Option<DateTime<Utc>>,
}

/// Parse JSONL history files for daily cost and session start time
pub fn parse_history(cwd: Option<&str>, session_id: Option<&str>) -> HistoryData {
    let mut data = HistoryData::default();
    let today = Utc::now().date_naive();

    let projects = match projects_dir() {
        Some(p) if p.exists() => p,
        _ => return data,
    };

    // Find the project folder for the current working directory
    let project_folder = cwd.and_then(|c| find_project_folder(&projects, c));

    // Collect all JSONL files to scan
    let jsonl_files: Vec<PathBuf> = WalkDir::new(&projects)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "jsonl"))
        .map(|e| e.path().to_path_buf())
        .collect();

    // Parse each file for daily cost
    for file in &jsonl_files {
        if let Ok(content) = std::fs::read_to_string(file) {
            for line in content.lines() {
                if let Some(entry) = parse_jsonl_line(line) {
                    // Add to daily cost if from today
                    if is_today(&entry.timestamp, today) {
                        data.daily_cost += entry.cost;
                    }

                    // Check for session start
                    if let Some(ref sid) = session_id {
                        if entry.session_id.as_deref() == Some(sid.as_ref()) {
                            if let Some(ts) = parse_timestamp(&entry.timestamp) {
                                match data.session_start {
                                    None => data.session_start = Some(ts),
                                    Some(existing) if ts < existing => {
                                        data.session_start = Some(ts)
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // If we have a specific project folder, also look for session start there
    if data.session_start.is_none() {
        if let Some(ref folder) = project_folder {
            data.session_start = find_session_start_in_folder(folder, session_id);
        }
    }

    data
}

#[derive(Debug, Deserialize)]
struct JsonlEntry {
    #[serde(default)]
    timestamp: String,
    #[serde(default, rename = "sessionId")]
    session_id: Option<String>,
    #[serde(default)]
    message: Option<MessageEntry>,
}

#[derive(Debug, Deserialize)]
struct MessageEntry {
    #[serde(default)]
    usage: Option<UsageEntry>,
}

#[derive(Debug, Deserialize)]
struct UsageEntry {
    #[serde(default, rename = "input_tokens")]
    input_tokens: u64,
    #[serde(default, rename = "output_tokens")]
    output_tokens: u64,
}

struct ParsedEntry {
    timestamp: String,
    session_id: Option<String>,
    cost: f64,
}

fn parse_jsonl_line(line: &str) -> Option<ParsedEntry> {
    let entry: JsonlEntry = serde_json::from_str(line).ok()?;

    let cost = entry
        .message
        .and_then(|m| m.usage)
        .map(|u| estimate_cost(u.input_tokens, u.output_tokens))
        .unwrap_or(0.0);

    Some(ParsedEntry {
        timestamp: entry.timestamp,
        session_id: entry.session_id,
        cost,
    })
}

/// Estimate cost based on Claude pricing (rough estimate)
fn estimate_cost(input_tokens: u64, output_tokens: u64) -> f64 {
    // Claude Sonnet 4 pricing: $3/1M input, $15/1M output
    let input_cost = (input_tokens as f64 / 1_000_000.0) * 3.0;
    let output_cost = (output_tokens as f64 / 1_000_000.0) * 15.0;
    input_cost + output_cost
}

fn is_today(timestamp: &str, today: NaiveDate) -> bool {
    parse_timestamp(timestamp)
        .map(|dt| dt.date_naive() == today)
        .unwrap_or(false)
}

fn parse_timestamp(ts: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(ts)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Map cwd to project folder path
fn find_project_folder(projects: &PathBuf, cwd: &str) -> Option<PathBuf> {
    // Claude stores projects as: ~/.claude/projects/-path-with-dashes/
    let folder_name = format!("-{}", cwd.replace('/', "-"));
    let folder = projects.join(&folder_name);
    if folder.exists() {
        Some(folder)
    } else {
        None
    }
}

fn find_session_start_in_folder(
    folder: &PathBuf,
    session_id: Option<&str>,
) -> Option<DateTime<Utc>> {
    let sid = session_id?;
    let mut earliest: Option<DateTime<Utc>> = None;

    for entry in WalkDir::new(folder)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "jsonl"))
    {
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            for line in content.lines() {
                if let Ok(entry) = serde_json::from_str::<JsonlEntry>(line) {
                    if entry.session_id.as_deref() == Some(sid) {
                        if let Some(ts) = parse_timestamp(&entry.timestamp) {
                            match earliest {
                                None => earliest = Some(ts),
                                Some(e) if ts < e => earliest = Some(ts),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    earliest
}
