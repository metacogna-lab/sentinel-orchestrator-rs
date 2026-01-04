pub mod auth;
pub mod error;
pub mod traits;
pub mod types;

// Re-export commonly used types
pub use auth::{ApiKey, ApiKeyId, AuthLevel, AuthResult};
pub use error::SentinelError;
pub use traits::{LLMProvider, VectorStore};
pub use types::{AgentId, AgentState, CanonicalMessage, MessageId, Role};
