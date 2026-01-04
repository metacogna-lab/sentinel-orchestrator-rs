// Medium-term memory implementation using Sled embedded database
// Stores conversation summaries that survive process restarts

use crate::core::error::SentinelError;
use crate::core::types::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{debug, error, warn};

/// Conversation summary stored in medium-term memory
/// This represents a condensed version of a conversation for persistent storage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationSummary {
    /// Agent ID this summary belongs to
    pub agent_id: AgentId,
    /// Unique conversation identifier
    pub conversation_id: String,
    /// Summarized content of the conversation
    pub summary: String,
    /// Number of messages that were summarized
    pub message_count: u64,
    /// When this summary was created
    pub created_at: DateTime<Utc>,
    /// When this summary was last updated
    pub last_updated: DateTime<Utc>,
}

impl ConversationSummary {
    /// Create a new conversation summary
    pub fn new(
        agent_id: AgentId,
        conversation_id: String,
        summary: String,
        message_count: u64,
    ) -> Self {
        let now = Utc::now();
        Self {
            agent_id,
            conversation_id,
            summary,
            message_count,
            created_at: now,
            last_updated: now,
        }
    }

    /// Update the summary content and timestamp
    pub fn update_summary(&mut self, summary: String, message_count: u64) {
        self.summary = summary;
        self.message_count = message_count;
        self.last_updated = Utc::now();
    }

    /// Serialize summary to bytes using bincode
    fn to_bytes(&self) -> Result<Vec<u8>, SentinelError> {
        bincode::serialize(self).map_err(|e| SentinelError::InvalidMessage {
            reason: format!("Serialization error: {}", e),
        })
    }

    /// Deserialize summary from bytes using bincode
    fn from_bytes(data: &[u8]) -> Result<Self, SentinelError> {
        bincode::deserialize(data).map_err(|e| SentinelError::InvalidMessage {
            reason: format!("Deserialization error: {}", e),
        })
    }

    /// Generate the storage key for this summary
    fn storage_key(&self) -> String {
        format!("{}:{}", self.agent_id, self.conversation_id)
    }

    /// Generate a storage key from components
    fn key_from_parts(agent_id: AgentId, conversation_id: &str) -> String {
        format!("{}:{}", agent_id, conversation_id)
    }
}

/// Medium-term memory using Sled embedded database
/// Provides persistent storage for conversation summaries
pub struct MediumTermMemory {
    db: sled::Db,
    path: PathBuf,
}

impl MediumTermMemory {
    /// Create a new medium-term memory instance
    ///
    /// # Arguments
    /// * `path` - Path to the Sled database directory
    ///
    /// # Returns
    /// * `Ok(MediumTermMemory)` - Successfully created
    /// * `Err(SentinelError)` - Error if database creation fails
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, SentinelError> {
        let path_buf = path.as_ref().to_path_buf();
        let db = sled::open(&path_buf).map_err(|e| SentinelError::DomainViolation {
            rule: format!("Failed to open Sled database at {:?}: {}", path_buf, e),
        })?;

        debug!("Opened medium-term memory database at {:?}", path_buf);

        Ok(Self {
            db,
            path: path_buf,
        })
    }

    /// Store a conversation summary
    ///
    /// # Arguments
    /// * `summary` - The conversation summary to store
    ///
    /// # Returns
    /// * `Ok(())` - Successfully stored
    /// * `Err(SentinelError)` - Error if storage fails
    pub fn store_summary(&self, summary: ConversationSummary) -> Result<(), SentinelError> {
        let key = summary.storage_key();
        let bytes = summary.to_bytes()?;

        self.db
            .insert(key.as_bytes(), bytes)
            .map_err(|e| SentinelError::DomainViolation {
                rule: format!("Failed to store summary {}: {}", key, e),
            })?;

        debug!("Stored conversation summary: {}", key);
        Ok(())
    }

    /// Retrieve a conversation summary
    ///
    /// # Arguments
    /// * `agent_id` - The agent ID
    /// * `conversation_id` - The conversation ID
    ///
    /// # Returns
    /// * `Ok(Some(ConversationSummary))` - Summary found
    /// * `Ok(None)` - Summary not found
    /// * `Err(SentinelError)` - Error if retrieval fails
    pub fn get_summary(
        &self,
        agent_id: AgentId,
        conversation_id: &str,
    ) -> Result<Option<ConversationSummary>, SentinelError> {
        let key = ConversationSummary::key_from_parts(agent_id, conversation_id);

        match self.db.get(key.as_bytes()) {
            Ok(Some(bytes)) => {
                let summary = ConversationSummary::from_bytes(&bytes)?;
                Ok(Some(summary))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(SentinelError::DomainViolation {
                rule: format!("Failed to retrieve summary {}: {}", key, e),
            }),
        }
    }

    /// List all conversation summaries for an agent
    ///
    /// # Arguments
    /// * `agent_id` - The agent ID to list summaries for
    ///
    /// # Returns
    /// * `Ok(Vec<ConversationSummary>)` - List of summaries
    /// * `Err(SentinelError)` - Error if listing fails
    pub fn list_summaries(&self, agent_id: AgentId) -> Result<Vec<ConversationSummary>, SentinelError> {
        let prefix = format!("{}:", agent_id);
        let mut summaries = Vec::new();

        for result in self.db.scan_prefix(prefix.as_bytes()) {
            match result {
                Ok((_key, bytes)) => {
                    match ConversationSummary::from_bytes(&bytes) {
                        Ok(summary) => summaries.push(summary),
                        Err(e) => {
                            warn!("Failed to deserialize summary: {}", e);
                            // Continue processing other summaries
                        }
                    }
                }
                Err(e) => {
                    error!("Error scanning summaries: {}", e);
                    return Err(SentinelError::DomainViolation {
                        rule: format!("Failed to scan summaries: {}", e),
                    });
                }
            }
        }

        debug!("Listed {} summaries for agent {}", summaries.len(), agent_id);
        Ok(summaries)
    }

    /// Delete a conversation summary
    ///
    /// # Arguments
    /// * `agent_id` - The agent ID
    /// * `conversation_id` - The conversation ID
    ///
    /// # Returns
    /// * `Ok(())` - Successfully deleted (or didn't exist)
    /// * `Err(SentinelError)` - Error if deletion fails
    pub fn delete_summary(
        &self,
        agent_id: AgentId,
        conversation_id: &str,
    ) -> Result<(), SentinelError> {
        let key = ConversationSummary::key_from_parts(agent_id, conversation_id);

        self.db
            .remove(key.as_bytes())
            .map_err(|e| SentinelError::DomainViolation {
                rule: format!("Failed to delete summary {}: {}", key, e),
            })?;

        debug!("Deleted conversation summary: {}", key);
        Ok(())
    }

    /// Get the database path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Flush all pending writes to disk
    ///
    /// # Returns
    /// * `Ok(())` - Successfully flushed
    /// * `Err(SentinelError)` - Error if flush fails
    pub fn flush(&self) -> Result<(), SentinelError> {
        self.db.flush().map_err(|e| SentinelError::DomainViolation {
            rule: format!("Failed to flush database: {}", e),
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_memory() -> (TempDir, MediumTermMemory) {
        let temp_dir = tempfile::tempdir().unwrap();
        let memory = MediumTermMemory::new(temp_dir.path()).unwrap();
        (temp_dir, memory)
    }

    #[test]
    fn test_store_and_retrieve() {
        let (_temp_dir, memory) = create_test_memory();

        let agent_id = AgentId::new();
        let summary = ConversationSummary::new(
            agent_id,
            "conv-1".to_string(),
            "Test summary".to_string(),
            10,
        );

        memory.store_summary(summary.clone()).unwrap();

        let retrieved = memory.get_summary(agent_id, "conv-1").unwrap();
        assert!(retrieved.is_some());
        let retrieved_summary = retrieved.unwrap();
        assert_eq!(retrieved_summary.agent_id, summary.agent_id);
        assert_eq!(retrieved_summary.conversation_id, summary.conversation_id);
        assert_eq!(retrieved_summary.summary, summary.summary);
        assert_eq!(retrieved_summary.message_count, summary.message_count);
    }

    #[test]
    fn test_get_missing_summary() {
        let (_temp_dir, memory) = create_test_memory();

        let agent_id = AgentId::new();
        let retrieved = memory.get_summary(agent_id, "nonexistent").unwrap();

        assert!(retrieved.is_none());
    }

    #[test]
    fn test_list_summaries() {
        let (_temp_dir, memory) = create_test_memory();

        let agent_id = AgentId::new();
        let summary1 = ConversationSummary::new(
            agent_id,
            "conv-1".to_string(),
            "Summary 1".to_string(),
            5,
        );
        let summary2 = ConversationSummary::new(
            agent_id,
            "conv-2".to_string(),
            "Summary 2".to_string(),
            8,
        );

        memory.store_summary(summary1).unwrap();
        memory.store_summary(summary2).unwrap();

        let summaries = memory.list_summaries(agent_id).unwrap();
        assert_eq!(summaries.len(), 2);
    }

    #[test]
    fn test_list_summaries_multiple_agents() {
        let (_temp_dir, memory) = create_test_memory();

        let agent_id1 = AgentId::new();
        let agent_id2 = AgentId::new();

        let summary1 = ConversationSummary::new(
            agent_id1,
            "conv-1".to_string(),
            "Summary 1".to_string(),
            5,
        );
        let summary2 = ConversationSummary::new(
            agent_id2,
            "conv-1".to_string(),
            "Summary 2".to_string(),
            8,
        );

        memory.store_summary(summary1).unwrap();
        memory.store_summary(summary2).unwrap();

        let summaries1 = memory.list_summaries(agent_id1).unwrap();
        assert_eq!(summaries1.len(), 1);

        let summaries2 = memory.list_summaries(agent_id2).unwrap();
        assert_eq!(summaries2.len(), 1);
    }

    #[test]
    fn test_delete_summary() {
        let (_temp_dir, memory) = create_test_memory();

        let agent_id = AgentId::new();
        let summary = ConversationSummary::new(
            agent_id,
            "conv-1".to_string(),
            "Test summary".to_string(),
            10,
        );

        memory.store_summary(summary).unwrap();
        assert!(memory.get_summary(agent_id, "conv-1").unwrap().is_some());

        memory.delete_summary(agent_id, "conv-1").unwrap();
        assert!(memory.get_summary(agent_id, "conv-1").unwrap().is_none());
    }

    #[test]
    fn test_delete_nonexistent_summary() {
        let (_temp_dir, memory) = create_test_memory();

        let agent_id = AgentId::new();
        // Should not error when deleting nonexistent summary
        memory.delete_summary(agent_id, "nonexistent").unwrap();
    }

    #[test]
    fn test_serialization_round_trip() {
        let agent_id = AgentId::new();
        let original = ConversationSummary::new(
            agent_id,
            "conv-1".to_string(),
            "Test summary content".to_string(),
            15,
        );

        let bytes = original.to_bytes().unwrap();
        let deserialized = ConversationSummary::from_bytes(&bytes).unwrap();

        assert_eq!(original.agent_id, deserialized.agent_id);
        assert_eq!(original.conversation_id, deserialized.conversation_id);
        assert_eq!(original.summary, deserialized.summary);
        assert_eq!(original.message_count, deserialized.message_count);
        assert_eq!(original.created_at, deserialized.created_at);
        assert_eq!(original.last_updated, deserialized.last_updated);
    }

    #[test]
    fn test_update_summary() {
        let mut summary = ConversationSummary::new(
            AgentId::new(),
            "conv-1".to_string(),
            "Original summary".to_string(),
            10,
        );

        let original_updated = summary.last_updated;
        std::thread::sleep(std::time::Duration::from_millis(10));

        summary.update_summary("Updated summary".to_string(), 20);

        assert_eq!(summary.summary, "Updated summary");
        assert_eq!(summary.message_count, 20);
        assert!(summary.last_updated > original_updated);
    }

    #[test]
    fn test_storage_key_format() {
        let agent_id = AgentId::new();
        let summary = ConversationSummary::new(
            agent_id,
            "conv-123".to_string(),
            "Test".to_string(),
            5,
        );

        let key = summary.storage_key();
        assert!(key.contains(&agent_id.to_string()));
        assert!(key.contains("conv-123"));
        assert!(key.contains(':'));
    }

    #[test]
    fn test_flush() {
        let (_temp_dir, memory) = create_test_memory();

        let agent_id = AgentId::new();
        let summary = ConversationSummary::new(
            agent_id,
            "conv-1".to_string(),
            "Test summary".to_string(),
            10,
        );

        memory.store_summary(summary).unwrap();
        memory.flush().unwrap(); // Should not panic
    }
}

