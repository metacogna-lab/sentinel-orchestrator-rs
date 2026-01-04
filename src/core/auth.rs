// Authentication and authorization domain types
// Pure domain logic with no external I/O dependencies

use serde::{Deserialize, Serialize};
use std::fmt;

/// API key identifier (NewType pattern for type safety)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApiKeyId(pub String);

impl ApiKeyId {
    /// Create a new API key ID
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Validate API key ID format
    ///
    /// # Validation Rules
    /// - Must not be empty
    /// - Must be between 1 and 255 characters
    /// - Must contain only alphanumeric characters, hyphens, and underscores
    ///
    /// # Returns
    /// * `Ok(())` - API key ID is valid
    /// * `Err(String)` - Validation error message
    pub fn validate(&self) -> Result<(), String> {
        if self.0.is_empty() {
            return Err("API key ID cannot be empty".to_string());
        }

        if self.0.len() > 255 {
            return Err("API key ID cannot exceed 255 characters".to_string());
        }

        // Allow alphanumeric, hyphens, and underscores
        if !self
            .0
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err("API key ID contains invalid characters".to_string());
        }

        Ok(())
    }
}

impl fmt::Display for ApiKeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// API key value (sensitive data)
/// This is a domain type representing an API key, but actual storage/validation
/// happens in the adapter layer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiKey(pub String);

impl ApiKey {
    /// Create a new API key
    pub fn new(key: String) -> Self {
        Self(key)
    }

    /// Validate API key format
    ///
    /// # Validation Rules
    /// - Must not be empty
    /// - Must be at least 16 characters (security requirement)
    /// - Must not exceed 512 characters
    ///
    /// # Returns
    /// * `Ok(())` - API key format is valid
    /// * `Err(String)` - Validation error message
    pub fn validate_format(&self) -> Result<(), String> {
        if self.0.is_empty() {
            return Err("API key cannot be empty".to_string());
        }

        if self.0.len() < 16 {
            return Err("API key must be at least 16 characters".to_string());
        }

        if self.0.len() > 512 {
            return Err("API key cannot exceed 512 characters".to_string());
        }

        Ok(())
    }

    /// Get the key value (for use in adapters only)
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Authentication result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthResult {
    /// Authentication successful
    Authenticated {
        /// API key ID of the authenticated key
        key_id: ApiKeyId,
    },
    /// Authentication failed
    Unauthenticated {
        /// Reason for authentication failure
        reason: String,
    },
}

/// Authorization level for API keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthLevel {
    /// Read-only access
    Read,
    /// Read and write access
    Write,
    /// Full administrative access
    Admin,
}

impl AuthLevel {
    /// Check if this auth level can perform a given action
    pub fn can_read(&self) -> bool {
        matches!(self, AuthLevel::Read | AuthLevel::Write | AuthLevel::Admin)
    }

    /// Check if this auth level can write
    pub fn can_write(&self) -> bool {
        matches!(self, AuthLevel::Write | AuthLevel::Admin)
    }

    /// Check if this auth level has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self, AuthLevel::Admin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_id_validation() {
        // Valid IDs
        assert!(ApiKeyId::new("test-key-123".to_string()).validate().is_ok());
        assert!(ApiKeyId::new("test_key_123".to_string()).validate().is_ok());
        assert!(ApiKeyId::new("testkey123".to_string()).validate().is_ok());
        assert!(ApiKeyId::new("a".to_string()).validate().is_ok());

        // Invalid: empty
        assert!(ApiKeyId::new("".to_string()).validate().is_err());

        // Invalid: too long
        let long_id = "a".repeat(256);
        assert!(ApiKeyId::new(long_id).validate().is_err());

        // Invalid: special characters
        assert!(ApiKeyId::new("test@key".to_string()).validate().is_err());
        assert!(ApiKeyId::new("test key".to_string()).validate().is_err());
    }

    #[test]
    fn test_api_key_format_validation() {
        // Valid keys
        assert!(ApiKey::new("sk-1234567890123456".to_string())
            .validate_format()
            .is_ok());
        let long_key = "a".repeat(16);
        assert!(ApiKey::new(long_key).validate_format().is_ok());

        // Invalid: too short
        assert!(ApiKey::new("short".to_string()).validate_format().is_err());

        // Invalid: empty
        assert!(ApiKey::new("".to_string()).validate_format().is_err());
    }

    #[test]
    fn test_auth_result() {
        let key_id = ApiKeyId::new("test-key".to_string());
        let authenticated = AuthResult::Authenticated {
            key_id: key_id.clone(),
        };

        match authenticated {
            AuthResult::Authenticated { key_id: id } => {
                assert_eq!(id, key_id);
            }
            _ => panic!("Expected Authenticated"),
        }

        let unauthenticated = AuthResult::Unauthenticated {
            reason: "Invalid key".to_string(),
        };

        match unauthenticated {
            AuthResult::Unauthenticated { reason } => {
                assert_eq!(reason, "Invalid key");
            }
            _ => panic!("Expected Unauthenticated"),
        }
    }

    #[test]
    fn test_auth_level_permissions() {
        // Read level
        assert!(AuthLevel::Read.can_read());
        assert!(!AuthLevel::Read.can_write());
        assert!(!AuthLevel::Read.is_admin());

        // Write level
        assert!(AuthLevel::Write.can_read());
        assert!(AuthLevel::Write.can_write());
        assert!(!AuthLevel::Write.is_admin());

        // Admin level
        assert!(AuthLevel::Admin.can_read());
        assert!(AuthLevel::Admin.can_write());
        assert!(AuthLevel::Admin.is_admin());
    }

    #[test]
    fn test_api_key_id_display() {
        let key_id = ApiKeyId::new("test-key-123".to_string());
        assert_eq!(format!("{}", key_id), "test-key-123");
    }
}
