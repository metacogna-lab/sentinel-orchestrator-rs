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

    /// Authentication failed
    #[error("Authentication failed: {reason}")]
    AuthenticationFailed {
        /// Reason for authentication failure
        reason: String,
    },

    /// Authorization failed (insufficient permissions)
    #[error("Authorization failed: {reason}")]
    AuthorizationFailed {
        /// Reason for authorization failure
        reason: String,
    },

    /// Invalid API key format
    #[error("Invalid API key format: {reason}")]
    InvalidApiKeyFormat {
        /// Reason why the API key format is invalid
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_state_transition_error() {
        let error = SentinelError::InvalidStateTransition {
            from: AgentState::Idle,
            to: AgentState::Reflecting,
        };

        match &error {
            SentinelError::InvalidStateTransition { from, to } => {
                assert_eq!(*from, AgentState::Idle);
                assert_eq!(*to, AgentState::Reflecting);
            }
            _ => panic!("Expected InvalidStateTransition"),
        }

        assert!(error.to_string().contains("Invalid state transition"));
        assert!(error.to_string().contains("Idle"));
        assert!(error.to_string().contains("Reflecting"));
    }

    #[test]
    fn test_invalid_message_error() {
        let reason = "Empty content not allowed".to_string();
        let error = SentinelError::InvalidMessage {
            reason: reason.clone(),
        };

        match &error {
            SentinelError::InvalidMessage { reason: r } => {
                assert_eq!(r, &reason);
            }
            _ => panic!("Expected InvalidMessage"),
        }

        assert!(error.to_string().contains("Invalid message"));
        assert!(error.to_string().contains("Empty content not allowed"));
    }

    #[test]
    fn test_domain_violation_error() {
        let rule = "Message must have non-empty content".to_string();
        let error = SentinelError::DomainViolation { rule: rule.clone() };

        match &error {
            SentinelError::DomainViolation { rule: r } => {
                assert_eq!(r, &rule);
            }
            _ => panic!("Expected DomainViolation"),
        }

        assert!(error.to_string().contains("Domain violation"));
        assert!(error
            .to_string()
            .contains("Message must have non-empty content"));
    }

    #[test]
    fn test_authentication_failed_error() {
        let error = SentinelError::AuthenticationFailed {
            reason: "Invalid API key".to_string(),
        };

        match &error {
            SentinelError::AuthenticationFailed { reason } => {
                assert_eq!(reason, "Invalid API key");
            }
            _ => panic!("Expected AuthenticationFailed"),
        }

        assert!(error.to_string().contains("Authentication failed"));
        assert!(error.to_string().contains("Invalid API key"));
    }

    #[test]
    fn test_authorization_failed_error() {
        let error = SentinelError::AuthorizationFailed {
            reason: "Insufficient permissions".to_string(),
        };

        match &error {
            SentinelError::AuthorizationFailed { reason } => {
                assert_eq!(reason, "Insufficient permissions");
            }
            _ => panic!("Expected AuthorizationFailed"),
        }

        assert!(error.to_string().contains("Authorization failed"));
        assert!(error.to_string().contains("Insufficient permissions"));
    }

    #[test]
    fn test_invalid_api_key_format_error() {
        let error = SentinelError::InvalidApiKeyFormat {
            reason: "Key too short".to_string(),
        };

        match &error {
            SentinelError::InvalidApiKeyFormat { reason } => {
                assert_eq!(reason, "Key too short");
            }
            _ => panic!("Expected InvalidApiKeyFormat"),
        }

        assert!(error.to_string().contains("Invalid API key format"));
        assert!(error.to_string().contains("Key too short"));
    }

    #[test]
    fn test_error_implements_error_trait() {
        let error = SentinelError::InvalidMessage {
            reason: "test".to_string(),
        };

        // Verify it implements std::error::Error
        let error_ref: &dyn std::error::Error = &error;
        assert!(error_ref.to_string().contains("Invalid message"));
    }

    #[test]
    fn test_error_clone() {
        let error1 = SentinelError::InvalidStateTransition {
            from: AgentState::Thinking,
            to: AgentState::ToolCall,
        };

        let error2 = error1.clone();
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_partial_eq() {
        let error1 = SentinelError::DomainViolation {
            rule: "test rule".to_string(),
        };
        let error2 = SentinelError::DomainViolation {
            rule: "test rule".to_string(),
        };
        let error3 = SentinelError::DomainViolation {
            rule: "different rule".to_string(),
        };

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_all_error_variants() {
        // Test that all variants can be created and display correctly
        let errors = vec![
            SentinelError::InvalidStateTransition {
                from: AgentState::Idle,
                to: AgentState::Thinking,
            },
            SentinelError::InvalidMessage {
                reason: "test".to_string(),
            },
            SentinelError::DomainViolation {
                rule: "test".to_string(),
            },
            SentinelError::AuthenticationFailed {
                reason: "Invalid API key".to_string(),
            },
            SentinelError::AuthorizationFailed {
                reason: "Insufficient permissions".to_string(),
            },
            SentinelError::InvalidApiKeyFormat {
                reason: "Key too short".to_string(),
            },
        ];

        for error in errors {
            let display = error.to_string();
            assert!(!display.is_empty());
            assert!(display.len() > 5); // Should have meaningful content
        }
    }
}
