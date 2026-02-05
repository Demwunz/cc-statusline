/// Shorten a path for display using fish-style abbreviation.
/// Intermediate directories are shortened to their first character,
/// the final component is kept in full. Home directory becomes ~.
/// e.g. /Users/fazal/dev/cc-statusline → ~/d/cc-statusline
pub fn shorten_path(path: &str) -> String {
    let mut p = path.to_string();

    // Replace home directory with ~
    if let Some(home) = dirs::home_dir() {
        if let Some(home_str) = home.to_str() {
            if p.starts_with(home_str) {
                p = format!("~{}", &p[home_str.len()..]);
            }
        }
    }

    // Split into components and shorten intermediates
    let parts: Vec<&str> = p.split('/').collect();
    if parts.len() <= 2 {
        return p;
    }

    let mut result = Vec::with_capacity(parts.len());
    let last = parts.len() - 1;
    for (i, part) in parts.iter().enumerate() {
        if i == last || *part == "~" || part.is_empty() {
            result.push(part.to_string());
        } else {
            // First character (handle multi-byte correctly)
            result.push(part.chars().next().unwrap().to_string());
        }
    }
    result.join("/")
}
