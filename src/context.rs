/// Context breakdown calculation

/// Base system prompt overhead (estimated, for segment visualization only)
pub const BASE_SYSTEM_TOKENS: u64 = 5000;

/// Default context window size
pub const DEFAULT_CONTEXT_WINDOW: u64 = 200_000;

/// Context breakdown for rendering the bar
///
/// Note: Claude Code's total_input_tokens already includes system prompt, skills, plugins,
/// and MCP overhead. We use the Claude Code tokens as the authoritative total, and the
/// estimated segments are for visualization purposes only (showing approximate composition).
#[derive(Debug, Clone)]
pub struct ContextBreakdown {
    /// Estimated base system tokens (for segment visualization)
    pub base_tokens: u64,
    /// Estimated skills tokens (for segment visualization)
    pub skills_tokens: u64,
    /// Estimated plugins tokens (for segment visualization)
    pub plugins_tokens: u64,
    /// Estimated MCP tokens (for segment visualization)
    pub mcp_tokens: u64,
    /// Actual total tokens from Claude Code (authoritative)
    pub total_tokens: u64,
    /// Context window size
    pub context_window: u64,
    /// Pre-calculated usage percentage from Claude Code (if available)
    pub used_percentage: Option<u64>,
    /// Remaining tokens in context window
    pub remaining_tokens: u64,
}

impl ContextBreakdown {
    pub fn new(
        skills_tokens: u64,
        plugins_tokens: u64,
        mcp_tokens: u64,
        input_tokens: u64,
        output_tokens: u64,
        context_window: Option<u64>,
        used_percentage: Option<u64>,
        remaining_tokens: u64,
    ) -> Self {
        // Claude Code's tokens are the authoritative total
        let total_from_claude = input_tokens + output_tokens;

        Self {
            base_tokens: BASE_SYSTEM_TOKENS,
            skills_tokens,
            plugins_tokens,
            mcp_tokens,
            total_tokens: total_from_claude,
            context_window: context_window.unwrap_or(DEFAULT_CONTEXT_WINDOW),
            used_percentage,
            remaining_tokens,
        }
    }

    /// Total tokens used (from Claude Code, authoritative)
    pub fn total(&self) -> u64 {
        self.total_tokens
    }

    /// Usage percentage (0-100, capped)
    pub fn percentage(&self) -> u64 {
        if let Some(pct) = self.used_percentage {
            return pct;
        }
        if self.context_window == 0 {
            return 0;
        }
        (self.total_tokens * 100) / self.context_window
    }

    /// Get segment sizes for visualization
    /// These are estimates scaled to fit within the actual total
    pub fn segments(&self) -> Vec<(u64, &'static str)> {
        if self.total_tokens == 0 {
            return vec![];
        }

        // Calculate estimated overhead
        let overhead = self.base_tokens + self.skills_tokens + self.plugins_tokens + self.mcp_tokens;

        // If total from Claude is less than our overhead estimate, scale down the segments
        // Otherwise, the remaining is conversation tokens
        if self.total_tokens <= overhead {
            // Scale all segments proportionally to fit
            let scale = self.total_tokens as f64 / overhead as f64;
            vec![
                ((self.base_tokens as f64 * scale) as u64, "base"),
                ((self.skills_tokens as f64 * scale) as u64, "skills"),
                ((self.plugins_tokens as f64 * scale) as u64, "plugins"),
                ((self.mcp_tokens as f64 * scale) as u64, "mcp"),
            ]
        } else {
            // Overhead fits, rest is conversation
            let conversation = self.total_tokens - overhead;
            vec![
                (self.base_tokens, "base"),
                (self.skills_tokens, "skills"),
                (self.plugins_tokens, "plugins"),
                (self.mcp_tokens, "mcp"),
                (conversation, "conversation"),
            ]
        }
    }
}
