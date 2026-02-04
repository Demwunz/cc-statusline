use std::path::Path;
use std::process::Command;

/// Git repository status
#[derive(Debug, Default)]
pub struct GitStatus {
    /// Branch name (or short commit hash if detached)
    pub branch: Option<String>,
    /// Number of staged changes
    pub staged: usize,
    /// Number of unstaged changes
    pub unstaged: usize,
    /// Number of untracked files
    pub untracked: usize,
    /// Commits ahead of upstream
    pub ahead: usize,
    /// Commits behind upstream
    pub behind: usize,
}

impl GitStatus {
    /// Check if the working tree is clean
    pub fn is_clean(&self) -> bool {
        self.staged == 0 && self.unstaged == 0 && self.untracked == 0
    }

    /// Check if we're in a git repo
    pub fn is_repo(&self) -> bool {
        self.branch.is_some()
    }
}

/// Get git status for the given directory
pub fn get_status(cwd: Option<&str>) -> GitStatus {
    let dir = match cwd {
        Some(d) => d,
        None => return GitStatus::default(),
    };

    let path = Path::new(dir);
    if !path.exists() {
        return GitStatus::default();
    }

    // Check if we're in a git repo and get branch name
    let branch = get_branch(dir);
    if branch.is_none() {
        return GitStatus::default();
    }

    let (staged, unstaged, untracked) = get_file_status(dir);
    let (ahead, behind) = get_ahead_behind(dir);

    GitStatus {
        branch,
        staged,
        unstaged,
        untracked,
        ahead,
        behind,
    }
}

/// Get current branch name or commit hash if detached
fn get_branch(dir: &str) -> Option<String> {
    // Try symbolic-ref first (for branch name)
    let output = Command::new("git")
        .args(["symbolic-ref", "--short", "HEAD"])
        .current_dir(dir)
        .output()
        .ok()?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !branch.is_empty() {
            return Some(branch);
        }
    }

    // Detached HEAD - get short commit hash
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(dir)
        .output()
        .ok()?;

    if output.status.success() {
        let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !hash.is_empty() {
            return Some(format!(":{}", hash));
        }
    }

    None
}

/// Get staged, unstaged, and untracked counts
fn get_file_status(dir: &str) -> (usize, usize, usize) {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(dir)
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => return (0, 0, 0),
    };

    let mut staged = 0;
    let mut unstaged = 0;
    let mut untracked = 0;

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.len() < 2 {
            continue;
        }
        let index = line.chars().next().unwrap_or(' ');
        let worktree = line.chars().nth(1).unwrap_or(' ');

        // Untracked
        if index == '?' {
            untracked += 1;
            continue;
        }

        // Staged (index has changes)
        if index != ' ' && index != '?' {
            staged += 1;
        }

        // Unstaged (worktree has changes)
        if worktree != ' ' && worktree != '?' {
            unstaged += 1;
        }
    }

    (staged, unstaged, untracked)
}

/// Get ahead/behind counts relative to upstream
fn get_ahead_behind(dir: &str) -> (usize, usize) {
    let output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", "@{upstream}...HEAD"])
        .current_dir(dir)
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => return (0, 0),
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = text.trim().split_whitespace().collect();

    if parts.len() == 2 {
        let behind = parts[0].parse().unwrap_or(0);
        let ahead = parts[1].parse().unwrap_or(0);
        (ahead, behind)
    } else {
        (0, 0)
    }
}
