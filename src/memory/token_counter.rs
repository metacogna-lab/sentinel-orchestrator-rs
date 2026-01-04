// Token counting implementation for accurate memory management
// Supports multiple counting strategies (simple approximation, accurate tokenization)

use crate::core::types::CanonicalMessage;

/// Trait for token counting strategies
/// Different implementations can provide varying levels of accuracy
pub trait TokenCounter: Send + Sync {
    /// Count tokens in a text string
    ///
    /// # Arguments
    /// * `text` - Text to count tokens for
    ///
    /// # Returns
    /// Number of tokens (approximate or accurate)
    fn count_tokens(&self, text: &str) -> u64;

    /// Count tokens in a single message
    ///
    /// # Arguments
    /// * `msg` - Message to count tokens for
    ///
    /// # Returns
    /// Number of tokens in the message
    fn count_message(&self, msg: &CanonicalMessage) -> u64 {
        self.count_tokens(&msg.content)
    }

    /// Count tokens across multiple messages
    ///
    /// # Arguments
    /// * `messages` - Slice of messages to count
    ///
    /// # Returns
    /// Total number of tokens across all messages
    fn count_messages(&self, messages: &[CanonicalMessage]) -> u64 {
        messages.iter().map(|msg| self.count_message(msg)).sum()
    }
}

/// Simple token counter using character approximation
/// Tokens ≈ characters / 4 (rough approximation for English text)
/// This is fast but not accurate for all languages or tokenization schemes
pub struct SimpleTokenCounter;

impl TokenCounter for SimpleTokenCounter {
    fn count_tokens(&self, text: &str) -> u64 {
        // Simple approximation: 1 token ≈ 4 characters
        // This is a rough estimate that works reasonably well for English
        text.chars().count() as u64 / 4
    }
}

impl Default for SimpleTokenCounter {
    fn default() -> Self {
        Self
    }
}

/// Accurate token counter (placeholder for future implementation)
/// This would use tiktoken or similar library for accurate tokenization
/// For now, it uses the same simple approximation
pub struct AccurateTokenCounter {
    // Future: tokenizer instance
    // For now, we'll use simple approximation
}

impl AccurateTokenCounter {
    /// Create a new accurate token counter
    pub fn new() -> Self {
        Self {}
    }

    /// Create with specific model (future implementation)
    pub fn with_model(_model: &str) -> Self {
        // Future: Initialize tokenizer for specific model
        Self {}
    }
}

impl TokenCounter for AccurateTokenCounter {
    fn count_tokens(&self, text: &str) -> u64 {
        // For now, use simple approximation
        // Future: Use actual tokenizer (tiktoken, etc.)
        text.chars().count() as u64 / 4
    }
}

impl Default for AccurateTokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Role;

    #[test]
    fn test_simple_token_counter() {
        let counter = SimpleTokenCounter;

        // Empty string
        assert_eq!(counter.count_tokens(""), 0);

        // Short text
        let text = "Hello";
        let tokens = counter.count_tokens(text);
        assert_eq!(tokens, text.chars().count() as u64 / 4);

        // Longer text
        let text = "This is a longer piece of text that should result in more tokens.";
        let tokens = counter.count_tokens(text);
        assert!(tokens > 0);
        assert_eq!(tokens, text.chars().count() as u64 / 4);
    }

    #[test]
    fn test_count_message() {
        let counter = SimpleTokenCounter;

        let message = CanonicalMessage::new(Role::User, "Hello, world!".to_string());

        let tokens = counter.count_message(&message);
        assert_eq!(tokens, "Hello, world!".chars().count() as u64 / 4);
    }

    #[test]
    fn test_count_messages() {
        let counter = SimpleTokenCounter;

        let messages = vec![
            CanonicalMessage::new(Role::User, "Hello".to_string()),
            CanonicalMessage::new(Role::Assistant, "Hi there!".to_string()),
            CanonicalMessage::new(Role::User, "How are you?".to_string()),
        ];

        let total_tokens = counter.count_messages(&messages);
        let expected: u64 = messages
            .iter()
            .map(|m| m.content.chars().count() as u64 / 4)
            .sum();

        assert_eq!(total_tokens, expected);
    }

    #[test]
    fn test_accurate_token_counter() {
        let counter = AccurateTokenCounter::new();

        // For now, should behave like simple counter
        let text = "Test text";
        let tokens = counter.count_tokens(text);
        assert_eq!(tokens, text.chars().count() as u64 / 4);
    }

    #[test]
    fn test_token_counter_trait_object() {
        // Test that we can use trait objects
        let counter: Box<dyn TokenCounter> = Box::new(SimpleTokenCounter);
        let tokens = counter.count_tokens("Hello");
        assert!(tokens > 0);
    }
}
