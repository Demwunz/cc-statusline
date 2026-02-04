use crate::config::projects_dir;
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use walkdir::WalkDir;

/// Cost information from JSONL history
#[derive(Debug, Default)]
pub struct HistoryData {
    pub daily_cost: f64,
}

/// Parse JSONL history files for daily cost
pub fn parse_history(_cwd: Option<&str>, _session_id: Option<&str>) -> HistoryData {
    let mut data = HistoryData::default();
    let today = Utc::now().date_naive();

    let projects = match projects_dir() {
        Some(p) if p.exists() => p,
        _ => return data,
    };

    // Scan all JSONL files for today's costs
    for entry in WalkDir::new(&projects)
        .follow_links(true)
        .max_depth(3) // Don't go too deep
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "jsonl"))
    {
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            for line in content.lines() {
                if let Some(cost) = parse_line_cost(line, today) {
                    data.daily_cost += cost;
                }
            }
        }
    }

    data
}

#[derive(Debug, Deserialize)]
struct JsonlEntry {
    #[serde(default)]
    timestamp: String,
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

fn parse_line_cost(line: &str, today: NaiveDate) -> Option<f64> {
    let entry: JsonlEntry = serde_json::from_str(line).ok()?;

    // Check if this entry is from today
    if !is_today(&entry.timestamp, today) {
        return None;
    }

    // Calculate cost from usage
    let usage = entry.message?.usage?;
    Some(estimate_cost(usage.input_tokens, usage.output_tokens))
}

/// Estimate cost based on Claude pricing (rough estimate using Sonnet rates)
fn estimate_cost(input_tokens: u64, output_tokens: u64) -> f64 {
    // Claude Sonnet 4 pricing: $3/1M input, $15/1M output
    let input_cost = (input_tokens as f64 / 1_000_000.0) * 3.0;
    let output_cost = (output_tokens as f64 / 1_000_000.0) * 15.0;
    input_cost + output_cost
}

fn is_today(timestamp: &str, today: NaiveDate) -> bool {
    // Try to parse ISO 8601 timestamp and check date
    timestamp
        .get(..10)
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .is_some_and(|d| d == today)
}
