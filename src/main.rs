mod colors;
mod config;
mod context;
mod history;
mod render;
mod scanner;

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
    Claude Code pipes JSON:
    {{\"model\": \"...\", \"inputTokens\": N, \"outputTokens\": N, \"contextWindow\": N, \"cost\": N}}

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

/// Input format from Claude Code
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StdinInput {
    model: Option<String>,
    #[serde(default)]
    input_tokens: u64,
    #[serde(default)]
    output_tokens: u64,
    #[serde(default)]
    context_window: Option<u64>,
    #[serde(default)]
    cost: f64,
    #[serde(default)]
    cwd: Option<String>,
    #[serde(default)]
    session_id: Option<String>,
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

        // Get cached scanner results
        let (skills_tokens, plugins_tokens, mcp_tokens) = cache.get();

        // Build context breakdown
        let breakdown = ContextBreakdown::new(
            skills_tokens,
            plugins_tokens,
            mcp_tokens,
            input.input_tokens,
            input.output_tokens,
            input.context_window,
        );

        // Get history data (session start, daily cost)
        let history = history::parse_history(input.cwd.as_deref(), input.session_id.as_deref());

        // Render and output
        let output = render::render(
            input.model.as_deref(),
            &breakdown,
            history.session_start,
            input.cost,
            history.daily_cost,
            config,
        );

        println!("{}", output);
    }
}
