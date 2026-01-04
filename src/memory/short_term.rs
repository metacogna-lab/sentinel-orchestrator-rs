// Short-term memory implementation
// In-memory conversation history with token counting and consolidation triggers

use crate::core::error::SentinelError;
use crate::core::types::{CanonicalMessage, Role};
use std::sync::{Arc, RwLock};

/// Default maximum number of messages in short-term memory
pub const DEFAULT_MAX_MESSAGES: usize = 1000;

/// Default maximum token count before consolidation
pub const DEFAULT_MAX_TOKENS: u64 = 100_000;

/// Default consolidation threshold (50k tokens)
pub const DEFAULT_CONSOLIDATION_THRESHOLD: u64 = 50_000;

/// Simple token counter using character approximation
/// Tokens ≈ characters / 4 (rough approximation)
fn approximate_tokens(text: &str) -> u64 {
    text.chars().count() as u64 / 4
}

/// Short-term memory for in-memory conversation history
/// This is the first tier of the three-tier memory hierarchy
pub struct ShortTermMemory {
    messages: Vec<CanonicalMessage>,
    token_count: u64,
    max_messages: usize,
    max_tokens: u64,
    consolidation_threshold: u64,
}

impl ShortTermMemory {
    /// Create a new short-term memory with default limits
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            token_count: 0,
            max_messages: DEFAULT_MAX_MESSAGES,
            max_tokens: DEFAULT_MAX_TOKENS,
            consolidation_threshold: DEFAULT_CONSOLIDATION_THRESHOLD,
        }
    }

    /// Create a new short-term memory with custom limits
    pub fn with_limits(max_messages: usize, max_tokens: u64, consolidation_threshold: u64) -> Self {
        Self {
            messages: Vec::new(),
            token_count: 0,
            max_messages,
            max_tokens,
            consolidation_threshold,
        }
    }

    /// Append a message to the conversation history
    ///
    /// # Arguments
    /// * `msg` - The message to append
    ///
    /// # Returns
    /// * `Ok(())` - Message appended successfully
    /// * `Err(SentinelError)` - Error if memory limits exceeded
    ///
    /// # Errors
    /// Returns `DomainViolation` if memory limits would be exceeded
    pub fn append_message(&mut self, msg: CanonicalMessage) -> Result<(), SentinelError> {
        let msg_tokens = approximate_tokens(&msg.content);

        // Check if adding this message would exceed limits
        if self.messages.len() >= self.max_messages {
            return Err(SentinelError::DomainViolation {
                rule: format!(
                    "Message limit exceeded: {} >= {}",
                    self.messages.len(),
                    self.max_messages
                ),
            });
        }

        if self.token_count + msg_tokens > self.max_tokens {
            return Err(SentinelError::DomainViolation {
                rule: format!(
                    "Token limit would be exceeded: {} + {} > {}",
                    self.token_count, msg_tokens, self.max_tokens
                ),
            });
        }

        // Add message and update token count
        self.token_count += msg_tokens;
        self.messages.push(msg);

        Ok(())
    }

    /// Get all messages in the conversation history
    ///
    /// # Returns
    /// Vector of all messages in chronological order
    pub fn get_messages(&self) -> Vec<CanonicalMessage> {
        self.messages.clone()
    }

    /// Get the most recent N messages
    ///
    /// # Arguments
    /// * `count` - Number of recent messages to retrieve
    ///
    /// # Returns
    /// Vector of the most recent messages (up to `count`)
    pub fn get_recent_messages(&self, count: usize) -> Vec<CanonicalMessage> {
        let start = self.messages.len().saturating_sub(count);
        self.messages[start..].to_vec()
    }

    /// Clear all messages and reset token count
    ///
    /// # Returns
    /// * `Ok(())` - Successfully cleared
    /// * `Err(SentinelError)` - Error if operation fails
    pub fn clear(&mut self) -> Result<(), SentinelError> {
        self.messages.clear();
        self.token_count = 0;
        Ok(())
    }

    /// Get the current number of messages
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Get the current token count
    pub fn token_count(&self) -> u64 {
        self.token_count
    }

    /// Check if consolidation should be triggered
    ///
    /// # Returns
    /// `true` if token count exceeds consolidation threshold
    pub fn should_consolidate(&self) -> bool {
        self.token_count >= self.consolidation_threshold
    }

    /// Get the consolidation threshold
    pub fn consolidation_threshold(&self) -> u64 {
        self.consolidation_threshold
    }

    /// Check if memory is at capacity (within 10% of limits)
    pub fn is_near_capacity(&self) -> bool {
        let message_ratio = self.messages.len() as f64 / self.max_messages as f64;
        let token_ratio = self.token_count as f64 / self.max_tokens as f64;
        message_ratio > 0.9 || token_ratio > 0.9
    }
}

impl Default for ShortTermMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe wrapper for short-term memory
/// Uses `Arc<RwLock<>>` for shared access
pub type SharedShortTermMemory = Arc<RwLock<ShortTermMemory>>;

/// Create a new shared short-term memory instance
pub fn create_shared_memory() -> SharedShortTermMemory {
    Arc::new(RwLock::new(ShortTermMemory::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_message() {
        let mut memory = ShortTermMemory::new();
        let msg = CanonicalMessage::new(Role::User, "test".to_string());
        memory.append_message(msg).unwrap();
        assert_eq!(memory.message_count(), 1);
    }

    #[test]
    fn test_append_multiple_messages() {
        let mut memory = ShortTermMemory::new();
        let msg1 = CanonicalMessage::new(Role::User, "message 1".to_string());
        let msg2 = CanonicalMessage::new(Role::Assistant, "message 2".to_string());
        let msg3 = CanonicalMessage::new(Role::User, "message 3".to_string());

        memory.append_message(msg1).unwrap();
        memory.append_message(msg2).unwrap();
        memory.append_message(msg3).unwrap();

        assert_eq!(memory.message_count(), 3);
        let messages = memory.get_messages();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].role, Role::User);
        assert_eq!(messages[1].role, Role::Assistant);
        assert_eq!(messages[2].role, Role::User);
    }

    #[test]
    fn test_token_count_updates() {
        let mut memory = ShortTermMemory::new();
        let msg = CanonicalMessage::new(Role::User, "This is a test message".to_string());

        let initial_count = memory.token_count();
        memory.append_message(msg).unwrap();

        assert!(memory.token_count() > initial_count);
    }

    #[test]
    fn test_should_consolidate() {
        let mut memory = ShortTermMemory::with_limits(1000, 100_000, 100); // Low threshold for testing

        // Add messages until threshold is reached
        let mut total_tokens = 0;
        while total_tokens < 100 {
            let msg = CanonicalMessage::new(
                Role::User,
                "This is a test message that has some tokens".to_string(),
            );
            let msg_tokens = approximate_tokens(&msg.content);
            if total_tokens + msg_tokens <= 100 {
                memory.append_message(msg).unwrap();
                total_tokens = memory.token_count();
            } else {
                break;
            }
        }

        assert!(memory.should_consolidate());
    }

    #[test]
    fn test_get_recent_messages() {
        let mut memory = ShortTermMemory::new();

        // Add 10 messages
        for i in 0..10 {
            let msg = CanonicalMessage::new(Role::User, format!("message {}", i));
            memory.append_message(msg).unwrap();
        }

        // Get last 3 messages
        let recent = memory.get_recent_messages(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].content, "message 7");
        assert_eq!(recent[1].content, "message 8");
        assert_eq!(recent[2].content, "message 9");
    }

    #[test]
    fn test_get_recent_messages_more_than_available() {
        let mut memory = ShortTermMemory::new();

        // Add 2 messages
        let msg1 = CanonicalMessage::new(Role::User, "message 1".to_string());
        let msg2 = CanonicalMessage::new(Role::User, "message 2".to_string());
        memory.append_message(msg1).unwrap();
        memory.append_message(msg2).unwrap();

        // Request 10 messages, should get only 2
        let recent = memory.get_recent_messages(10);
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut memory = ShortTermMemory::new();

        // Add some messages
        for i in 0..5 {
            let msg = CanonicalMessage::new(Role::User, format!("message {}", i));
            memory.append_message(msg).unwrap();
        }

        assert_eq!(memory.message_count(), 5);
        assert!(memory.token_count() > 0);

        memory.clear().unwrap();

        assert_eq!(memory.message_count(), 0);
        assert_eq!(memory.token_count(), 0);
    }

    #[test]
    fn test_message_limit_enforcement() {
        let mut memory = ShortTermMemory::with_limits(2, 100_000, 50_000);

        let msg1 = CanonicalMessage::new(Role::User, "message 1".to_string());
        let msg2 = CanonicalMessage::new(Role::User, "message 2".to_string());
        let msg3 = CanonicalMessage::new(Role::User, "message 3".to_string());

        memory.append_message(msg1).unwrap();
        memory.append_message(msg2).unwrap();

        // Third message should fail
        let result = memory.append_message(msg3);
        assert!(result.is_err());
        match result.unwrap_err() {
            SentinelError::DomainViolation { rule } => {
                assert!(rule.contains("Message limit exceeded"));
            }
            _ => panic!("Expected DomainViolation error"),
        }
    }

    #[test]
    fn test_token_limit_enforcement() {
        let mut memory = ShortTermMemory::with_limits(1000, 10, 50_000); // Very low token limit

        // First small message should work
        let msg1 = CanonicalMessage::new(Role::User, "hi".to_string());
        memory.append_message(msg1).unwrap();

        // Large message should fail
        let large_content = "x".repeat(100); // Will generate ~25 tokens
        let msg2 = CanonicalMessage::new(Role::User, large_content);
        let result = memory.append_message(msg2);
        assert!(result.is_err());
        match result.unwrap_err() {
            SentinelError::DomainViolation { rule } => {
                assert!(rule.contains("Token limit"));
            }
            _ => panic!("Expected DomainViolation error"),
        }
    }

    #[test]
    fn test_is_near_capacity() {
        let mut memory = ShortTermMemory::with_limits(100, 1000, 500);

        // Fill to 91% capacity (should trigger near capacity)
        for i in 0..91 {
            let msg = CanonicalMessage::new(Role::User, format!("msg {}", i));
            memory.append_message(msg).unwrap();
        }

        assert!(memory.is_near_capacity());
    }

    #[test]
    fn test_shared_memory() {
        let shared = create_shared_memory();

        {
            let mut memory = shared.write().unwrap();
            let msg = CanonicalMessage::new(Role::User, "test".to_string());
            memory.append_message(msg).unwrap();
        }

        {
            let memory = shared.read().unwrap();
            assert_eq!(memory.message_count(), 1);
        }
    }

    #[test]
    fn test_message_order_preserved() {
        let mut memory = ShortTermMemory::new();

        let messages: Vec<_> = (0..10)
            .map(|i| CanonicalMessage::new(Role::User, format!("message {}", i)))
            .collect();

        for msg in &messages {
            memory.append_message(msg.clone()).unwrap();
        }

        let retrieved = memory.get_messages();
        assert_eq!(retrieved.len(), messages.len());
        for (i, (retrieved_msg, original_msg)) in retrieved.iter().zip(messages.iter()).enumerate()
        {
            assert_eq!(
                retrieved_msg.content, original_msg.content,
                "Message {} out of order",
                i
            );
        }
    }
}
