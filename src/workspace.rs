use std::path::Path;

/// Detected project language/framework
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Rust,
    Go,
    TypeScript,
    JavaScript,
    Ruby,
    Elixir,
    Python,
    Unknown,
}

impl Language {
    /// Get display name for the language
    pub fn name(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Go => "go",
            Language::TypeScript => "ts",
            Language::JavaScript => "js",
            Language::Ruby => "rb",
            Language::Elixir => "ex",
            Language::Python => "py",
            Language::Unknown => "",
        }
    }

    /// Get color name for the language
    pub fn color(&self) -> &'static str {
        match self {
            Language::Rust => "red",
            Language::Go => "cyan",
            Language::TypeScript => "blue",
            Language::JavaScript => "yellow",
            Language::Ruby => "red",
            Language::Elixir => "magenta",
            Language::Python => "yellow",
            Language::Unknown => "gray",
        }
    }
}

/// Detect the primary language/framework of a project
pub fn detect_language(cwd: Option<&str>) -> Language {
    let dir = match cwd {
        Some(d) => d,
        None => return Language::Unknown,
    };

    let path = Path::new(dir);
    if !path.exists() {
        return Language::Unknown;
    }

    // Check for language-specific files (order matters - more specific first)
    if path.join("Cargo.toml").exists() {
        Language::Rust
    } else if path.join("go.mod").exists() {
        Language::Go
    } else if path.join("mix.exs").exists() {
        Language::Elixir
    } else if path.join("Gemfile").exists() {
        Language::Ruby
    } else if path.join("tsconfig.json").exists() {
        Language::TypeScript
    } else if path.join("package.json").exists() {
        // Could be TS or JS - check for tsconfig
        if path.join("tsconfig.json").exists() {
            Language::TypeScript
        } else {
            Language::JavaScript
        }
    } else if path.join("pyproject.toml").exists()
        || path.join("setup.py").exists()
        || path.join("requirements.txt").exists()
    {
        Language::Python
    } else {
        Language::Unknown
    }
}

/// Shorten a path for display (replace home with ~, truncate if needed)
pub fn shorten_path(path: &str) -> String {
    // Replace home directory with ~
    if let Some(home) = dirs::home_dir() {
        if let Some(home_str) = home.to_str() {
            if path.starts_with(home_str) {
                return format!("~{}", &path[home_str.len()..]);
            }
        }
    }
    path.to_string()
}
