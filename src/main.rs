mod colors;
mod config;
mod context;
mod git;
mod history;
mod render;
mod scanner;
mod workspace;

use config::Config;
use context::ContextBreakdown;
use serde::Deserialize;
use std::io::{self, BufRead};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Handle CLI flags
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("cc-statusline {}", VERSION);
        return;
    }

    let config = Config::load();

    if args.iter().any(|a| a == "--legend" || a == "-l") {
        render::print_legend(&config);
        return;
    }

    // Normal mode: read JSON from stdin
    run_statusline(&config);
}

fn print_help() {
    println!(
        "cc-statusline {}
Lightweight statusline for Claude Code

USAGE:
    cc-statusline [OPTIONS]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version
    -l, --legend     Show what the colors mean

STDIN FORMAT:
    Claude Code pipes nested JSON with model, context_window, cost, and workspace fields.

CONFIG:
    ~/.config/cc-statusline/config.toml

SETUP:
    Add to ~/.claude/settings.json:
    {{
      \"statusLine\": {{
        \"type\": \"command\",
        \"command\": \"cc-statusline\"
      }}
    }}",
        VERSION
    );
}

/// Input format from Claude Code (nested JSON structure)
#[derive(Debug, Deserialize)]
struct StdinInput {
    #[serde(default)]
    model: Option<ModelInfo>,
    #[serde(default)]
    context_window: Option<ContextWindowInfo>,
    #[serde(default)]
    cost: Option<CostInfo>,
    #[serde(default)]
    workspace: Option<WorkspaceInfo>,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    subagents: Option<SubagentInfo>,
}

#[derive(Debug, Deserialize)]
struct SubagentInfo {
    #[serde(default)]
    active: u32,
}

#[derive(Debug, Deserialize)]
struct ModelInfo {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ContextWindowInfo {
    #[serde(default)]
    total_input_tokens: u64,
    #[serde(default)]
    total_output_tokens: u64,
    #[serde(default)]
    context_window_size: u64,
    #[serde(default)]
    used_percentage: Option<f64>,
    #[serde(default)]
    remaining_percentage: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct CostInfo {
    #[serde(default)]
    total_cost_usd: f64,
    #[serde(default)]
    total_duration_ms: u64,
}

#[derive(Debug, Deserialize)]
struct WorkspaceInfo {
    #[serde(default)]
    current_dir: Option<String>,
}

fn run_statusline(config: &Config) {
    let stdin = io::stdin();
    let mut cache = scanner::ScanCache::new(config.cache.ttl_seconds);

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if line.trim().is_empty() {
            continue;
        }

        let input: StdinInput = match serde_json::from_str(&line) {
            Ok(i) => i,
            Err(_) => {
                // On parse error, output a simple fallback
                println!("[--] ░░░░░░░░░░░░░░░░░░░░░░░░ 0%");
                continue;
            }
        };

        // Extract nested values
        let model_name = input
            .model
            .as_ref()
            .and_then(|m| m.display_name.as_deref().or(m.id.as_deref()));

        let (input_tokens, output_tokens, context_size) = input
            .context_window
            .as_ref()
            .map(|c| (c.total_input_tokens, c.total_output_tokens, Some(c.context_window_size)))
            .unwrap_or((0, 0, None));

        let used_pct = input.context_window.as_ref().and_then(|c| {
            c.used_percentage.map(|p| p.round() as u64)
        });

        let remaining_tokens = input.context_window.as_ref().map(|c| {
            c.remaining_percentage
                .map(|pct| (c.context_window_size as f64 * pct / 100.0) as u64)
                .unwrap_or(0)
        }).unwrap_or(0);

        let session_cost = input.cost.as_ref().map(|c| c.total_cost_usd).unwrap_or(0.0);
        let duration_ms = input.cost.as_ref().map(|c| c.total_duration_ms).unwrap_or(0);

        let cwd = input.workspace.as_ref().and_then(|w| w.current_dir.clone());

        // Get subagent count
        let subagent_count = input.subagents.as_ref().map(|s| s.active).unwrap_or(0);

        // Get cached scanner results
        let (skills_tokens, plugins_tokens, mcp_tokens) = cache.get();

        // Build context breakdown
        let breakdown = ContextBreakdown::new(
            skills_tokens,
            plugins_tokens,
            mcp_tokens,
            input_tokens,
            output_tokens,
            context_size,
            used_pct,
            remaining_tokens,
        );

        // Get history data (daily cost)
        let history = history::parse_history(cwd.as_deref(), input.session_id.as_deref());

        // Get git status
        let git_status = git::get_status(cwd.as_deref());

        // Detect language
        let lang = workspace::detect_language(cwd.as_deref());

        // Render and output
        let output = render::render(
            model_name,
            &breakdown,
            duration_ms,
            session_cost,
            history.daily_cost,
            &git_status,
            &lang,
            cwd.as_deref(),
            subagent_count,
            config,
        );

        println!("{}", output);
    }
}
