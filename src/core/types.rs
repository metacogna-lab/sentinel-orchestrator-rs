// Domain types - CanonicalMessage, AgentState, etc.
// These are immutable contracts that define the domain model.
// Frontend must adhere to these types when interacting with the backend.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a message (NewType pattern)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// User-sent message
    User,
    /// Assistant/AI-generated message
    Assistant,
    /// System/context-setting message
    System,
}

/// Agent state in the state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

/// Canonical message format - pure domain type with no external dependencies
/// This is the immutable contract for all message communication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Health status
    pub status: HealthState,
    /// Timestamp of the health check
    pub timestamp: DateTime<Utc>,
}

/// Health state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Total tokens used
    pub total_tokens: u32,
}

/// Agent status information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Optional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
}
