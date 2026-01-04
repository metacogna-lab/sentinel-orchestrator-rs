// Consolidation trigger configuration and logic
// Defines when and how memory consolidation should occur

use std::time::Duration;

/// Consolidation trigger configuration
/// Defines thresholds and conditions for memory consolidation
#[derive(Debug, Clone)]
pub struct ConsolidationConfig {
    /// Token threshold for short-term memory consolidation
    pub short_term_token_threshold: u64,
    /// Message count threshold for short-term memory consolidation
    pub short_term_message_threshold: usize,
    /// Number of summaries before medium-term consolidation
    pub medium_term_summary_threshold: usize,
    /// Age threshold for medium-term summaries (older summaries prioritized)
    pub medium_term_age_threshold: Duration,
    /// Enable automatic consolidation in background
    pub enable_auto_consolidation: bool,
}

impl Default for ConsolidationConfig {
    fn default() -> Self {
        Self {
            short_term_token_threshold: 50_000,
            short_term_message_threshold: 1000,
            medium_term_summary_threshold: 10,
            medium_term_age_threshold: Duration::from_secs(86400), // 24 hours
            enable_auto_consolidation: true,
        }
    }
}

/// Consolidation priority levels
/// Higher priority consolidations are processed first
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConsolidationPriority {
    /// Critical: Memory overflow imminent, immediate action required
    Critical = 4,
    /// High: Threshold exceeded, consolidation needed soon
    High = 3,
    /// Medium: Normal consolidation trigger
    Medium = 2,
    /// Low: Maintenance consolidation, can be deferred
    Low = 1,
}

impl ConsolidationPriority {
    /// Get priority name for logging
    pub fn name(&self) -> &'static str {
        match self {
            ConsolidationPriority::Critical => "Critical",
            ConsolidationPriority::High => "High",
            ConsolidationPriority::Medium => "Medium",
            ConsolidationPriority::Low => "Low",
        }
    }
}

/// Token budget tracking across all memory tiers
#[derive(Debug, Clone)]
pub struct TokenBudget {
    /// Current tokens in short-term memory
    pub short_term_tokens: u64,
    /// Estimated tokens in medium-term memory (summaries)
    pub medium_term_tokens: u64,
    /// Estimated tokens in long-term memory (embeddings)
    pub long_term_tokens: u64,
    /// Maximum total tokens allowed (None = unlimited)
    pub max_total_tokens: Option<u64>,
}

impl TokenBudget {
    /// Create a new token budget tracker
    pub fn new() -> Self {
        Self {
            short_term_tokens: 0,
            medium_term_tokens: 0,
            long_term_tokens: 0,
            max_total_tokens: None,
        }
    }

    /// Create with a maximum total token limit
    pub fn with_limit(max_total: u64) -> Self {
        Self {
            short_term_tokens: 0,
            medium_term_tokens: 0,
            long_term_tokens: 0,
            max_total_tokens: Some(max_total),
        }
    }

    /// Get total tokens across all tiers
    pub fn total(&self) -> u64 {
        self.short_term_tokens + self.medium_term_tokens + self.long_term_tokens
    }

    /// Check if budget is exceeded
    pub fn exceeds_budget(&self) -> bool {
        if let Some(max) = self.max_total_tokens {
            self.total() > max
        } else {
            false
        }
    }

    /// Get remaining budget (None if unlimited)
    pub fn remaining(&self) -> Option<u64> {
        self.max_total_tokens.map(|max| {
            let total = self.total();
            max.saturating_sub(total)
        })
    }

    /// Get budget usage percentage (0-100)
    pub fn usage_percentage(&self) -> Option<f64> {
        self.max_total_tokens.map(|max| {
            let total = self.total();
            if max == 0 {
                0.0
            } else {
                (total as f64 / max as f64 * 100.0).min(100.0)
            }
        })
    }

    /// Update short-term token count
    pub fn update_short_term(&mut self, tokens: u64) {
        self.short_term_tokens = tokens;
    }

    /// Update medium-term token count
    pub fn update_medium_term(&mut self, tokens: u64) {
        self.medium_term_tokens = tokens;
    }

    /// Update long-term token count
    pub fn update_long_term(&mut self, tokens: u64) {
        self.long_term_tokens = tokens;
    }
}

impl Default for TokenBudget {
    fn default() -> Self {
        Self::new()
    }
}

/// Consolidation trigger evaluator
/// Determines when consolidation should occur based on configuration
pub struct ConsolidationTrigger {
    config: ConsolidationConfig,
}

impl ConsolidationTrigger {
    /// Create a new consolidation trigger with default config
    pub fn new() -> Self {
        Self {
            config: ConsolidationConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: ConsolidationConfig) -> Self {
        Self { config }
    }

    /// Check if short-term consolidation should occur
    ///
    /// # Arguments
    /// * `token_count` - Current token count in short-term memory
    /// * `message_count` - Current message count in short-term memory
    ///
    /// # Returns
    /// Priority level if consolidation needed, None otherwise
    pub fn should_consolidate_short(
        &self,
        token_count: u64,
        message_count: usize,
    ) -> Option<ConsolidationPriority> {
        // Critical: Token count exceeds threshold by 2x
        let critical_threshold = self.config.short_term_token_threshold.saturating_mul(2);
        if token_count >= critical_threshold {
            return Some(ConsolidationPriority::Critical);
        }

        // High: Token threshold exceeded
        if token_count >= self.config.short_term_token_threshold {
            return Some(ConsolidationPriority::High);
        }

        // High: Message count threshold exceeded
        if message_count >= self.config.short_term_message_threshold {
            return Some(ConsolidationPriority::High);
        }

        None
    }

    /// Check if medium-term consolidation should occur
    ///
    /// # Arguments
    /// * `summary_count` - Number of summaries in medium-term memory
    ///
    /// # Returns
    /// Priority level if consolidation needed, None otherwise
    pub fn should_consolidate_medium(&self, summary_count: usize) -> Option<ConsolidationPriority> {
        if summary_count >= self.config.medium_term_summary_threshold {
            Some(ConsolidationPriority::Medium)
        } else {
            None
        }
    }

    /// Get the consolidation configuration
    pub fn config(&self) -> &ConsolidationConfig {
        &self.config
    }
}

impl Default for ConsolidationTrigger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidation_config_default() {
        let config = ConsolidationConfig::default();
        assert_eq!(config.short_term_token_threshold, 50_000);
        assert_eq!(config.short_term_message_threshold, 1000);
        assert_eq!(config.medium_term_summary_threshold, 10);
        assert!(config.enable_auto_consolidation);
    }

    #[test]
    fn test_token_budget_total() {
        let mut budget = TokenBudget::new();
        budget.short_term_tokens = 1000;
        budget.medium_term_tokens = 2000;
        budget.long_term_tokens = 3000;

        assert_eq!(budget.total(), 6000);
    }

    #[test]
    fn test_token_budget_exceeds() {
        let mut budget = TokenBudget::with_limit(5000);
        budget.short_term_tokens = 3000;
        budget.medium_term_tokens = 2000;
        budget.long_term_tokens = 1000;

        assert!(budget.exceeds_budget());
        assert_eq!(budget.total(), 6000);
    }

    #[test]
    fn test_token_budget_remaining() {
        let mut budget = TokenBudget::with_limit(10000);
        budget.short_term_tokens = 2000;
        budget.medium_term_tokens = 3000;

        assert_eq!(budget.remaining(), Some(5000));
    }

    #[test]
    fn test_token_budget_usage_percentage() {
        let mut budget = TokenBudget::with_limit(10000);
        budget.short_term_tokens = 5000;

        let usage = budget.usage_percentage().unwrap();
        assert_eq!(usage, 50.0);
    }

    #[test]
    fn test_consolidation_trigger_short_term() {
        let trigger = ConsolidationTrigger::new();

        // No consolidation needed
        assert!(trigger.should_consolidate_short(1000, 10).is_none());

        // High priority: token threshold exceeded
        let priority = trigger.should_consolidate_short(60_000, 100);
        assert_eq!(priority, Some(ConsolidationPriority::High));

        // Critical: 2x threshold
        let priority = trigger.should_consolidate_short(120_000, 100);
        assert_eq!(priority, Some(ConsolidationPriority::Critical));

        // High priority: message threshold exceeded
        let priority = trigger.should_consolidate_short(1000, 1500);
        assert_eq!(priority, Some(ConsolidationPriority::High));
    }

    #[test]
    fn test_consolidation_trigger_medium_term() {
        let trigger = ConsolidationTrigger::new();

        // No consolidation needed
        assert!(trigger.should_consolidate_medium(5).is_none());

        // Medium priority: summary threshold exceeded
        let priority = trigger.should_consolidate_medium(15);
        assert_eq!(priority, Some(ConsolidationPriority::Medium));
    }

    #[test]
    fn test_consolidation_priority_ordering() {
        assert!(ConsolidationPriority::Critical > ConsolidationPriority::High);
        assert!(ConsolidationPriority::High > ConsolidationPriority::Medium);
        assert!(ConsolidationPriority::Medium > ConsolidationPriority::Low);
    }

    #[test]
    fn test_consolidation_priority_name() {
        assert_eq!(ConsolidationPriority::Critical.name(), "Critical");
        assert_eq!(ConsolidationPriority::High.name(), "High");
        assert_eq!(ConsolidationPriority::Medium.name(), "Medium");
        assert_eq!(ConsolidationPriority::Low.name(), "Low");
    }
}
