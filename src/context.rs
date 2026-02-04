/// Context breakdown calculation

/// Base system prompt overhead (estimated)
pub const BASE_SYSTEM_TOKENS: u64 = 5000;

/// Default context window size
pub const DEFAULT_CONTEXT_WINDOW: u64 = 200_000;

/// Context breakdown for rendering the bar
#[derive(Debug, Clone)]
pub struct ContextBreakdown {
    pub base_tokens: u64,
    pub skills_tokens: u64,
    pub plugins_tokens: u64,
    pub mcp_tokens: u64,
    pub conversation_tokens: u64,
    pub context_window: u64,
}

impl ContextBreakdown {
    pub fn new(
        skills_tokens: u64,
        plugins_tokens: u64,
        mcp_tokens: u64,
        input_tokens: u64,
        output_tokens: u64,
        context_window: Option<u64>,
    ) -> Self {
        Self {
            base_tokens: BASE_SYSTEM_TOKENS,
            skills_tokens,
            plugins_tokens,
            mcp_tokens,
            conversation_tokens: input_tokens + output_tokens,
            context_window: context_window.unwrap_or(DEFAULT_CONTEXT_WINDOW),
        }
    }

    /// Total tokens used
    pub fn total(&self) -> u64 {
        self.base_tokens
            + self.skills_tokens
            + self.plugins_tokens
            + self.mcp_tokens
            + self.conversation_tokens
    }

    /// Usage percentage (0-100)
    pub fn percentage(&self) -> u64 {
        if self.context_window == 0 {
            return 0;
        }
        (self.total() * 100) / self.context_window
    }

    /// Get segment sizes as fractions of the context window
    pub fn segments(&self) -> Vec<(u64, &'static str)> {
        vec![
            (self.base_tokens, "base"),
            (self.skills_tokens, "skills"),
            (self.plugins_tokens, "plugins"),
            (self.mcp_tokens, "mcp"),
            (self.conversation_tokens, "conversation"),
        ]
    }
}
