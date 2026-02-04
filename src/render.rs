use crate::colors::{self, BOLD, DIM, RESET};
use crate::config::Config;
use crate::context::ContextBreakdown;

const FILLED_CHAR: char = '█';
const EMPTY_CHAR: char = '░';

/// Render the statusline to a string
pub fn render(
    model: Option<&str>,
    breakdown: &ContextBreakdown,
    duration_ms: u64,
    session_cost: f64,
    daily_cost: f64,
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

    // Percentage
    output.push_str(&format!(" {}%", breakdown.percentage()));

    // Session time
    output.push_str(&format!(" {DIM}│{RESET} "));
    output.push_str(&format_duration_ms(duration_ms));

    // Cost
    if config.format.show_cost {
        output.push_str(&format!(" {DIM}│{RESET} "));
        output.push_str(&format_cost(session_cost));
        if config.format.show_daily_cost {
            output.push_str(&format!(" {DIM}/{RESET} "));
            output.push_str(&format_cost(daily_cost));
        }
    }

    output
}

/// Render the color-coded context bar
fn render_bar(breakdown: &ContextBreakdown, config: &Config) -> String {
    let width = if config.format.bar_width > 0 {
        config.format.bar_width
    } else {
        // Auto-detect: try to use terminal width, default to 24
        terminal_width().map(|w| w.min(40)).unwrap_or(24)
    };

    let total = breakdown.context_window;
    if total == 0 {
        return format!(
            "{}{}{}",
            colors::color_code(&config.colors.empty),
            EMPTY_CHAR.to_string().repeat(width),
            RESET
        );
    }

    let mut bar = String::new();
    let mut chars_used = 0;

    // Render each segment
    for (tokens, segment_type) in breakdown.segments() {
        if tokens == 0 {
            continue;
        }

        let fraction = tokens as f64 / total as f64;
        let segment_chars = ((fraction * width as f64).round() as usize).max(1);
        let chars_to_draw = segment_chars.min(width - chars_used);

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
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;

    if hours > 0 {
        format!("{BOLD}{}h{:02}m{RESET}", hours, mins)
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
    println!("Time:  session duration (from first message)");
    println!("Cost:  $session / $today");
}
