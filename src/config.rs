use serde::Deserialize;
use std::path::PathBuf;

/// Application configuration loaded from ~/.config/cc-statusline/config.toml
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub format: FormatConfig,
    pub colors: ColorConfig,
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct FormatConfig {
    pub show_model: bool,
    pub show_cost: bool,
    pub show_daily_cost: bool,
    pub bar_width: usize,
    /// Subscription plan: "api" (default, pay-per-token) or "max" (fixed monthly)
    pub plan: String,
    /// Monthly cost of subscription plan (e.g., 100.0 for $100/month Max plan)
    pub plan_cost: f64,
    /// Show git status (branch, dirty state)
    pub show_git: bool,
    /// Show detected language/framework
    pub show_lang: bool,
    /// Show current working directory
    pub show_cwd: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ColorConfig {
    pub base: String,
    pub skills: String,
    pub plugins: String,
    pub mcp: String,
    pub conversation: String,
    pub empty: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct CacheConfig {
    pub ttl_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: FormatConfig::default(),
            colors: ColorConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            show_model: true,
            show_cost: true,
            show_daily_cost: true,
            bar_width: 24,
            plan: "api".to_string(),
            plan_cost: 100.0, // Default Max plan cost
            show_git: true,
            show_lang: true,
            show_cwd: true,
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            base: "blue".to_string(),
            skills: "cyan".to_string(),
            plugins: "magenta".to_string(),
            mcp: "yellow".to_string(),
            conversation: "green".to_string(),
            empty: "gray".to_string(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self { ttl_seconds: 5 }
    }
}

impl Config {
    /// Load config from ~/.config/cc-statusline/config.toml
    pub fn load() -> Self {
        let path = config_path();
        if let Some(p) = path {
            if p.exists() {
                if let Ok(content) = std::fs::read_to_string(&p) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }
}

/// Returns the config file path
/// Checks ~/.config first (XDG style), then platform-specific config dir
fn config_path() -> Option<PathBuf> {
    // First try ~/.config (XDG style, cross-platform friendly)
    if let Some(home) = dirs::home_dir() {
        let xdg_path = home.join(".config").join("cc-statusline").join("config.toml");
        if xdg_path.exists() {
            return Some(xdg_path);
        }
    }
    // Fall back to platform-specific config dir
    dirs::config_dir().map(|c| c.join("cc-statusline").join("config.toml"))
}

/// Returns the Claude home directory (~/.claude)
pub fn claude_home() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".claude"))
}

/// Returns the skills directory (~/.claude/skills)
pub fn skills_dir() -> Option<PathBuf> {
    claude_home().map(|h| h.join("skills"))
}

/// Returns the plugins directory (~/.claude/plugins)
pub fn plugins_dir() -> Option<PathBuf> {
    claude_home().map(|h| h.join("plugins"))
}

/// Returns the settings file path (~/.claude/settings.json)
pub fn settings_path() -> Option<PathBuf> {
    claude_home().map(|h| h.join("settings.json"))
}

/// Returns the installed plugins file path
pub fn installed_plugins_path() -> Option<PathBuf> {
    plugins_dir().map(|p| p.join("installed_plugins.json"))
}

/// Returns the projects directory (~/.claude/projects)
pub fn projects_dir() -> Option<PathBuf> {
    claude_home().map(|h| h.join("projects"))
}
