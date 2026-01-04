// Domain-specific errors using thiserror
// This will be fully implemented in Phase 1, Item 3

use crate::core::types::AgentState;
use thiserror::Error;

/// Sentinel domain errors.
/// These represent errors that can occur in the domain layer.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SentinelError {
    /// Invalid state transition attempted
    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        /// Source state
        from: AgentState,
        /// Target state
        to: AgentState,
    },

    /// Invalid message format or content
    #[error("Invalid message: {reason}")]
    InvalidMessage {
        /// Reason why the message is invalid
        reason: String,
    },

    /// Domain rule violation
    #[error("Domain violation: {rule}")]
    DomainViolation {
        /// The rule that was violated
        rule: String,
    },
}
