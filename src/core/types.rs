// Domain types - CanonicalMessage, AgentState, etc.
// These are immutable contracts that define the domain model.
// Frontend must adhere to these types when interacting with the backend.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

/// Unique identifier for a message (NewType pattern)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct MessageId(pub Uuid);

impl MessageId {
    /// Generate a new MessageId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for MessageId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<MessageId> for Uuid {
    fn from(id: MessageId) -> Self {
        id.0
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for an agent/actor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct AgentId(pub Uuid);

impl AgentId {
    /// Generate a new AgentId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for AgentId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<AgentId> for Uuid {
    fn from(id: AgentId) -> Self {
        id.0
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Role of a message participant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(rename_all = "lowercase")]
pub enum Role {
    /// User-sent message
    User,
    /// Assistant/AI-generated message
    Assistant,
    /// System/context-setting message
    System,
}

/// Agent state in the state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(rename_all = "lowercase")]
pub enum AgentState {
    /// Agent is idle, waiting for messages
    Idle,
    /// Agent is processing/thinking
    Thinking,
    /// Agent is calling a tool
    ToolCall,
    /// Agent is reflecting on results
    Reflecting,
}

impl AgentState {
    /// Validate if a state transition is allowed
    ///
    /// # Arguments
    /// * `next` - The target state to transition to
    ///
    /// # Returns
    /// `true` if the transition is valid, `false` otherwise
    ///
    /// # State Machine Rules
    /// - Idle → Thinking (when message received)
    /// - Thinking → ToolCall (when tool needed)
    /// - Thinking → Reflecting (when processing complete)
    /// - ToolCall → Reflecting (after tool execution)
    /// - Reflecting → Idle (after reflection complete)
    /// - Idle → Idle (self-loop allowed)
    pub fn can_transition_to(&self, next: AgentState) -> bool {
        match (self, next) {
            // Valid transitions
            (AgentState::Idle, AgentState::Thinking) => true,
            (AgentState::Idle, AgentState::Idle) => true, // Self-loop allowed
            (AgentState::Thinking, AgentState::ToolCall) => true,
            (AgentState::Thinking, AgentState::Reflecting) => true,
            (AgentState::ToolCall, AgentState::Reflecting) => true,
            (AgentState::Reflecting, AgentState::Idle) => true,
            // Invalid transitions
            _ => false,
        }
    }

    /// Get all valid next states from the current state
    ///
    /// # Returns
    /// Vector of all valid states that can be transitioned to from the current state
    pub fn valid_next_states(&self) -> Vec<AgentState> {
        match self {
            AgentState::Idle => vec![AgentState::Idle, AgentState::Thinking],
            AgentState::Thinking => vec![AgentState::ToolCall, AgentState::Reflecting],
            AgentState::ToolCall => vec![AgentState::Reflecting],
            AgentState::Reflecting => vec![AgentState::Idle],
        }
    }

    /// Attempt to transition to a new state
    ///
    /// # Arguments
    /// * `next` - The target state to transition to
    ///
    /// # Returns
    /// * `Ok(AgentState)` - The new state if transition is valid
    /// * `Err(SentinelError)` - InvalidStateTransition error if transition is not allowed
    pub fn transition_to(
        &self,
        next: AgentState,
    ) -> Result<AgentState, crate::core::error::SentinelError> {
        if self.can_transition_to(next) {
            Ok(next)
        } else {
            Err(crate::core::error::SentinelError::InvalidStateTransition {
                from: *self,
                to: next,
            })
        }
    }
}

/// Canonical message format - pure domain type with no external dependencies
/// This is the immutable contract for all message communication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct CanonicalMessage {
    /// Unique identifier for this message
    pub id: MessageId,
    /// Role of the message sender
    pub role: Role,
    /// Message content
    pub content: String,
    /// Timestamp when message was created
    pub timestamp: DateTime<Utc>,
    /// Optional metadata (key-value pairs)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

impl CanonicalMessage {
    /// Create a new canonical message
    pub fn new(role: Role, content: String) -> Self {
        Self {
            id: MessageId::new(),
            role,
            content,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new canonical message with explicit timestamp (for replay/testing)
    pub fn with_timestamp(role: Role, content: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            id: MessageId::new(),
            role,
            content,
            timestamp,
            metadata: HashMap::new(),
        }
    }

    /// Create a new canonical message with metadata
    pub fn with_metadata(role: Role, content: String, metadata: HashMap<String, String>) -> Self {
        Self {
            id: MessageId::new(),
            role,
            content,
            timestamp: Utc::now(),
            metadata,
        }
    }
}

/// Health status response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct HealthStatus {
    /// Health status
    pub status: HealthState,
    /// Timestamp of the health check
    pub timestamp: DateTime<Utc>,
}

/// Health state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(rename_all = "lowercase")]
pub enum HealthState {
    /// System is healthy
    Healthy,
    /// System is ready (all components initialized)
    Ready,
    /// System is alive (basic liveness check)
    Alive,
    /// System is unhealthy
    Unhealthy,
}

/// Chat completion request (API contract)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChatCompletionRequest {
    /// List of messages in the conversation
    pub messages: Vec<CanonicalMessage>,
    /// Model to use (optional, defaults to configured model)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Temperature for sampling (0.0 to 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Stream responses
    #[serde(default)]
    pub stream: bool,
}

/// Chat completion response (API contract)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ChatCompletionResponse {
    /// Generated message
    pub message: CanonicalMessage,
    /// Model used for generation
    pub model: String,
    /// Number of tokens used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
}

/// Token usage information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct TokenUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Total tokens used
    pub total_tokens: u32,
}

/// Agent status information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct AgentStatus {
    /// Agent identifier
    pub id: AgentId,
    /// Current state
    pub state: AgentState,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Number of messages processed
    pub messages_processed: u64,
}

/// Error response format (API contract)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Optional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::error::SentinelError;

    #[test]
    fn test_valid_state_transitions() {
        // Idle → Thinking
        assert!(AgentState::Idle.can_transition_to(AgentState::Thinking));
        // Idle → Idle (self-loop)
        assert!(AgentState::Idle.can_transition_to(AgentState::Idle));
        // Thinking → ToolCall
        assert!(AgentState::Thinking.can_transition_to(AgentState::ToolCall));
        // Thinking → Reflecting
        assert!(AgentState::Thinking.can_transition_to(AgentState::Reflecting));
        // ToolCall → Reflecting
        assert!(AgentState::ToolCall.can_transition_to(AgentState::Reflecting));
        // Reflecting → Idle
        assert!(AgentState::Reflecting.can_transition_to(AgentState::Idle));
    }

    #[test]
    fn test_invalid_state_transitions() {
        // Idle → ToolCall (invalid, must go through Thinking)
        assert!(!AgentState::Idle.can_transition_to(AgentState::ToolCall));
        // Idle → Reflecting (invalid)
        assert!(!AgentState::Idle.can_transition_to(AgentState::Reflecting));
        // Thinking → Idle (invalid, must go through Reflecting)
        assert!(!AgentState::Thinking.can_transition_to(AgentState::Idle));
        // ToolCall → Thinking (invalid)
        assert!(!AgentState::ToolCall.can_transition_to(AgentState::Thinking));
        // ToolCall → Idle (invalid)
        assert!(!AgentState::ToolCall.can_transition_to(AgentState::Idle));
        // Reflecting → Thinking (invalid)
        assert!(!AgentState::Reflecting.can_transition_to(AgentState::Thinking));
        // Reflecting → ToolCall (invalid)
        assert!(!AgentState::Reflecting.can_transition_to(AgentState::ToolCall));
    }

    #[test]
    fn test_valid_next_states() {
        let idle_states = AgentState::Idle.valid_next_states();
        assert_eq!(idle_states.len(), 2);
        assert!(idle_states.contains(&AgentState::Idle));
        assert!(idle_states.contains(&AgentState::Thinking));

        let thinking_states = AgentState::Thinking.valid_next_states();
        assert_eq!(thinking_states.len(), 2);
        assert!(thinking_states.contains(&AgentState::ToolCall));
        assert!(thinking_states.contains(&AgentState::Reflecting));

        let toolcall_states = AgentState::ToolCall.valid_next_states();
        assert_eq!(toolcall_states.len(), 1);
        assert_eq!(toolcall_states[0], AgentState::Reflecting);

        let reflecting_states = AgentState::Reflecting.valid_next_states();
        assert_eq!(reflecting_states.len(), 1);
        assert_eq!(reflecting_states[0], AgentState::Idle);
    }

    #[test]
    fn test_transition_to_valid() {
        let result = AgentState::Idle.transition_to(AgentState::Thinking);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), AgentState::Thinking);

        let result = AgentState::Thinking.transition_to(AgentState::ToolCall);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), AgentState::ToolCall);

        let result = AgentState::Idle.transition_to(AgentState::Idle);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), AgentState::Idle);
    }

    #[test]
    fn test_transition_to_invalid() {
        let result = AgentState::Idle.transition_to(AgentState::ToolCall);
        assert!(result.is_err());
        match result.unwrap_err() {
            SentinelError::InvalidStateTransition { from, to } => {
                assert_eq!(from, AgentState::Idle);
                assert_eq!(to, AgentState::ToolCall);
            }
            _ => panic!("Expected InvalidStateTransition error"),
        }

        let result = AgentState::Reflecting.transition_to(AgentState::Thinking);
        assert!(result.is_err());
        match result.unwrap_err() {
            SentinelError::InvalidStateTransition { from, to } => {
                assert_eq!(from, AgentState::Reflecting);
                assert_eq!(to, AgentState::Thinking);
            }
            _ => panic!("Expected InvalidStateTransition error"),
        }
    }

    #[test]
    fn test_complete_state_cycle() {
        // Test a complete valid cycle: Idle → Thinking → ToolCall → Reflecting → Idle
        let mut state = AgentState::Idle;

        state = state.transition_to(AgentState::Thinking).unwrap();
        assert_eq!(state, AgentState::Thinking);

        state = state.transition_to(AgentState::ToolCall).unwrap();
        assert_eq!(state, AgentState::ToolCall);

        state = state.transition_to(AgentState::Reflecting).unwrap();
        assert_eq!(state, AgentState::Reflecting);

        state = state.transition_to(AgentState::Idle).unwrap();
        assert_eq!(state, AgentState::Idle);
    }

    #[test]
    fn test_alternative_path() {
        // Test alternative path: Idle → Thinking → Reflecting → Idle (skipping ToolCall)
        let mut state = AgentState::Idle;

        state = state.transition_to(AgentState::Thinking).unwrap();
        assert_eq!(state, AgentState::Thinking);

        state = state.transition_to(AgentState::Reflecting).unwrap();
        assert_eq!(state, AgentState::Reflecting);

        state = state.transition_to(AgentState::Idle).unwrap();
        assert_eq!(state, AgentState::Idle);
    }
}
