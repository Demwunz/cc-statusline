use crate::colors::{self, BOLD, DIM, RESET, GREEN, RED, CYAN};
use crate::config::Config;
use crate::context::ContextBreakdown;
use crate::git::GitStatus;
use crate::workspace::{Language, shorten_path};

const FILLED_CHAR: char = '█';
const EMPTY_CHAR: char = '░';

/// Render the statusline to a string
pub fn render(
    model: Option<&str>,
    breakdown: &ContextBreakdown,
    duration_ms: u64,
    session_cost: f64,
    daily_cost: f64,
    git: &GitStatus,
    lang: &Language,
    cwd: Option<&str>,
    subagent_count: u32,
    config: &Config,
) -> String {
    let mut output = String::new();

    // Model tag
    if config.format.show_model {
        if let Some(m) = model {
            let short = shorten_model(m);
            output.push_str(&format!("{DIM}[{short}]{RESET} "));
        }
    }

    // Context bar
    output.push_str(&render_bar(breakdown, config));

    // Percentage (cap at 100%, show "+" indicator if exceeded)
    let pct = breakdown.percentage();
    if pct > 100 {
        output.push_str(&format!(" {BOLD}100%+{RESET}"));
    } else {
        output.push_str(&format!(" {}%", pct));
    }

    // Subagent count (only show when > 0)
    if subagent_count > 0 {
        output.push_str(&format!(" {CYAN}⚡{}{RESET}", subagent_count));
    }

    // Session time (elapsed)
    output.push_str(&format!(" {DIM}│{RESET} "));
    output.push_str(&format_duration_ms(duration_ms));

    // Cost
    if config.format.show_cost {
        output.push_str(&format!(" {DIM}│{RESET} "));
        if config.format.plan == "max" {
            // Max plan: show plan name and savings when applicable
            output.push_str(&format!("{DIM}max{RESET}"));
            // Show savings when daily cost exceeds plan cost
            if daily_cost > config.format.plan_cost {
                let saved = daily_cost - config.format.plan_cost;
                output.push_str(&format!(" {DIM}({RESET}"));
                output.push_str(&format_cost(saved));
                output.push_str(&format!(" {DIM}saved){RESET}"));
            }
        } else {
            // API plan: show session cost / daily cost as before
            output.push_str(&format_cost(session_cost));
            if config.format.show_daily_cost {
                output.push_str(&format!(" {DIM}/{RESET} "));
                output.push_str(&format_cost(daily_cost));
            }
        }
    }

    // Git status
    if config.format.show_git && git.is_repo() {
        output.push_str(&format!(" {DIM}│{RESET} "));
        output.push_str(&render_git(git));
    }

    // Language
    if config.format.show_lang && *lang != Language::Unknown {
        output.push_str(&format!(" {DIM}│{RESET} "));
        let color = colors::color_code(lang.color());
        output.push_str(&format!("{}{}{}", color, lang.name(), RESET));
    }

    // Current working directory
    if config.format.show_cwd {
        if let Some(dir) = cwd {
            output.push_str(&format!(" {DIM}│{RESET} "));
            output.push_str(&format!("{DIM}{}{RESET}", shorten_path(dir)));
        }
    }

    output
}

/// Render git status with colors and symbols
fn render_git(git: &GitStatus) -> String {
    let mut parts = String::new();

    // Branch name - color based on clean/dirty state
    if let Some(ref branch) = git.branch {
        if git.is_clean() {
            parts.push_str(&format!("{GREEN}{}{RESET}", branch));
        } else {
            parts.push_str(&format!("{RED}{}{RESET}", branch));
        }
    }

    // Status indicators
    let mut indicators = String::new();

    // Ahead/behind
    if git.ahead > 0 {
        indicators.push_str(&format!("↑{}", git.ahead));
    }
    if git.behind > 0 {
        indicators.push_str(&format!("↓{}", git.behind));
    }

    // Changes: staged (✚), unstaged (●), untracked (?)
    if git.staged > 0 {
        indicators.push_str(&format!("{GREEN}✚{}{RESET}", git.staged));
    }
    if git.unstaged > 0 {
        indicators.push_str(&format!("{RED}●{}{RESET}", git.unstaged));
    }
    if git.untracked > 0 {
        indicators.push_str(&format!("{DIM}?{}{RESET}", git.untracked));
    }

    if !indicators.is_empty() {
        parts.push(' ');
        parts.push_str(&indicators);
    }

    parts
}

/// Render the color-coded context bar
fn render_bar(breakdown: &ContextBreakdown, config: &Config) -> String {
    let width = if config.format.bar_width > 0 {
        config.format.bar_width
    } else {
        // Auto-detect: try to use terminal width, default to 24
        terminal_width().map(|w| w.min(40)).unwrap_or(24)
    };

    let context_window = breakdown.context_window;
    if context_window == 0 {
        return format!(
            "{}{}{}",
            colors::color_code(&config.colors.empty),
            EMPTY_CHAR.to_string().repeat(width),
            RESET
        );
    }

    let mut bar = String::new();
    let mut chars_used = 0;

    // Calculate the total filled portion of the bar (capped at 100%)
    let total_tokens = breakdown.total();
    let fill_ratio = (total_tokens as f64 / context_window as f64).min(1.0);
    let total_filled_chars = (fill_ratio * width as f64).round() as usize;

    // Render each segment proportionally within the filled area
    // Segments are scaled relative to total tokens used (not context window)
    for (tokens, segment_type) in breakdown.segments() {
        if tokens == 0 || chars_used >= total_filled_chars {
            continue;
        }

        // Calculate this segment's proportion of the total used tokens
        let segment_fraction = tokens as f64 / total_tokens as f64;
        let segment_chars = ((segment_fraction * total_filled_chars as f64).round() as usize).max(1);
        let chars_to_draw = segment_chars.min(total_filled_chars - chars_used);

        if chars_to_draw == 0 {
            continue;
        }

        let color = match segment_type {
            "base" => colors::color_code(&config.colors.base),
            "skills" => colors::color_code(&config.colors.skills),
            "plugins" => colors::color_code(&config.colors.plugins),
            "mcp" => colors::color_code(&config.colors.mcp),
            "conversation" => colors::color_code(&config.colors.conversation),
            _ => colors::color_code(&config.colors.empty),
        };

        bar.push_str(color);
        bar.push_str(&FILLED_CHAR.to_string().repeat(chars_to_draw));
        bar.push_str(RESET);

        chars_used += chars_to_draw;
    }

    // Fill remaining with empty
    if chars_used < width {
        bar.push_str(colors::color_code(&config.colors.empty));
        bar.push_str(&EMPTY_CHAR.to_string().repeat(width - chars_used));
        bar.push_str(RESET);
    }

    bar
}

fn shorten_model(model: &str) -> &str {
    if model.contains("opus") {
        "opus"
    } else if model.contains("sonnet") {
        "sonnet"
    } else if model.contains("haiku") {
        "haiku"
    } else {
        // Return last part after hyphen, or first 8 chars
        model.rsplit('-').next().unwrap_or(&model[..8.min(model.len())])
    }
}

fn format_duration_ms(ms: u64) -> String {
    if ms == 0 {
        return format!("{DIM}0m{RESET}");
    }

    let total_secs = ms / 1000;
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;

    if days > 0 {
        format!("{BOLD}{}d {}h{RESET}", days, hours)
    } else if hours > 0 {
        format!("{BOLD}{}h {:02}m{RESET}", hours, mins)
    } else {
        format!("{BOLD}{}m{RESET}", mins)
    }
}

fn format_cost(cost: f64) -> String {
    if cost < 0.01 {
        format!("{DIM}$0.00{RESET}")
    } else if cost < 1.0 {
        format!("{BOLD}${:.2}{RESET}", cost)
    } else if cost < 10.0 {
        format!("{BOLD}${:.2}{RESET}", cost)
    } else {
        format!("{BOLD}${:.1}{RESET}", cost)
    }
}

fn terminal_width() -> Option<usize> {
    // Simple heuristic: check COLUMNS env var
    std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse().ok())
}

/// Print the legend explaining colors
pub fn print_legend(config: &Config) {
    println!("Context breakdown (left to right):\n");
    println!(
        "  {}{}{} blue    = base system (~5k tokens)",
        colors::color_code(&config.colors.base),
        FILLED_CHAR.to_string().repeat(2),
        RESET
    );
    println!(
        "  {}{}{} cyan    = skills",
        colors::color_code(&config.colors.skills),
        FILLED_CHAR.to_string().repeat(2),
        RESET
    );
    println!(
        "  {}{}{} magenta = plugins (enabled)",
        colors::color_code(&config.colors.plugins),
        FILLED_CHAR.to_string().repeat(2),
        RESET
    );
    println!(
        "  {}{}{} yellow  = MCP servers",
        colors::color_code(&config.colors.mcp),
        FILLED_CHAR.to_string().repeat(2),
        RESET
    );
    println!(
        "  {}{}{} green   = conversation",
        colors::color_code(&config.colors.conversation),
        FILLED_CHAR.to_string().repeat(2),
        RESET
    );
    println!(
        "  {}{}{} gray    = available",
        colors::color_code(&config.colors.empty),
        EMPTY_CHAR.to_string().repeat(2),
        RESET
    );
    println!();
    println!("Time:  elapsed session time (format: Xd Xh or Xh XXm)");
    println!("Cost:  $session / $today (api plan) or \"max\" with savings (max plan)");
    println!();
    println!("Git status:");
    println!("  {GREEN}branch{RESET}  = clean working tree");
    println!("  {RED}branch{RESET}  = dirty working tree");
    println!("  ↑n      = commits ahead of upstream");
    println!("  ↓n      = commits behind upstream");
    println!("  {GREEN}✚n{RESET}      = staged changes");
    println!("  {RED}●n{RESET}      = unstaged changes");
    println!("  {DIM}?n{RESET}      = untracked files");
    println!();
    println!("Language: detected from project files (Cargo.toml, package.json, etc.)");
    println!("Path:     current working directory (~ = home)");
    println!();
    println!("Subagents:");
    println!("  {CYAN}⚡n{RESET}     = active subagents (shown only when > 0)");
}
