/// ANSI escape codes for terminal colors

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";

pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const GRAY: &str = "\x1b[90m";

/// Get ANSI color code from color name
pub fn color_code(name: &str) -> &'static str {
    match name.to_lowercase().as_str() {
        "black" => BLACK,
        "red" => RED,
        "green" => GREEN,
        "yellow" => YELLOW,
        "blue" => BLUE,
        "magenta" => MAGENTA,
        "cyan" => CYAN,
        "white" => WHITE,
        "gray" | "grey" => GRAY,
        _ => WHITE,
    }
}
