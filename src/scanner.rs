use crate::config::{installed_plugins_path, settings_path, skills_dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use walkdir::WalkDir;

/// Cached scan results with TTL
pub struct ScanCache {
    skills_tokens: u64,
    plugins_tokens: u64,
    mcp_tokens: u64,
    last_scan: Instant,
    ttl: Duration,
}

impl ScanCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            skills_tokens: 0,
            plugins_tokens: 0,
            mcp_tokens: 0,
            last_scan: Instant::now() - Duration::from_secs(ttl_seconds + 1),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Get cached values, refreshing if stale
    pub fn get(&mut self) -> (u64, u64, u64) {
        if self.last_scan.elapsed() > self.ttl {
            self.refresh();
        }
        (self.skills_tokens, self.plugins_tokens, self.mcp_tokens)
    }

    fn refresh(&mut self) {
        self.skills_tokens = scan_skills();
        self.plugins_tokens = scan_plugins();
        self.mcp_tokens = scan_mcp();
        self.last_scan = Instant::now();
    }
}

/// Scan ~/.claude/skills and return total tokens
fn scan_skills() -> u64 {
    let path = match skills_dir() {
        Some(p) if p.exists() => p,
        _ => return 0,
    };

    let total_bytes: u64 = std::fs::read_dir(&path)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
        .map(|e| calculate_dir_size(&e.path()))
        .sum();

    bytes_to_tokens(total_bytes)
}

/// Scan plugins and return tokens for enabled plugins
fn scan_plugins() -> u64 {
    let settings = load_settings();
    let installed = load_installed_plugins();

    let total_bytes: u64 = installed
        .iter()
        .filter(|p| {
            let key = if let Some(ref m) = p.marketplace {
                format!("{}@{}", p.entry, m)
            } else {
                p.entry.clone()
            };
            settings.enabled_plugins.get(&key).copied().unwrap_or(true)
        })
        .filter_map(|p| p.install_path.as_ref())
        .map(|path| {
            let p = PathBuf::from(path);
            if p.exists() {
                calculate_dir_size(&p)
            } else {
                0
            }
        })
        .sum();

    bytes_to_tokens(total_bytes)
}

/// Scan MCP servers and return tokens (500 per server)
fn scan_mcp() -> u64 {
    let settings = load_settings();
    settings.mcp_servers.len() as u64 * 500
}

fn calculate_dir_size(path: &std::path::Path) -> u64 {
    WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

fn bytes_to_tokens(bytes: u64) -> u64 {
    bytes / 4
}

// Settings structures
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeSettings {
    #[serde(default)]
    mcp_servers: HashMap<String, serde_json::Value>,
    #[serde(default)]
    enabled_plugins: HashMap<String, bool>,
}

fn load_settings() -> ClaudeSettings {
    settings_path()
        .filter(|p| p.exists())
        .and_then(|p| std::fs::read_to_string(&p).ok())
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_default()
}

#[derive(Debug, Deserialize)]
struct InstalledPluginsFile {
    #[serde(default)]
    plugins: HashMap<String, Vec<PluginRecord>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PluginRecord {
    install_path: Option<String>,
}

struct InstalledPlugin {
    entry: String,
    marketplace: Option<String>,
    install_path: Option<String>,
}

fn load_installed_plugins() -> Vec<InstalledPlugin> {
    let path = match installed_plugins_path() {
        Some(p) if p.exists() => p,
        _ => return Vec::new(),
    };

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let file: InstalledPluginsFile = match serde_json::from_str(&content) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    file.plugins
        .into_iter()
        .filter_map(|(entry, records)| {
            let record = records.into_iter().next()?;
            let parts: Vec<&str> = entry.split('@').collect();
            let (name, marketplace) = if parts.len() >= 2 {
                (parts[0].to_string(), Some(parts[1].to_string()))
            } else {
                (entry.clone(), None)
            };
            Some(InstalledPlugin {
                entry: name,
                marketplace,
                install_path: record.install_path,
            })
        })
        .collect()
}
