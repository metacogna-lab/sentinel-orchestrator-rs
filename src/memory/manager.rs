// Memory hierarchy management (Short/Med/Long term)
// The Dreamer - coordinates the three-tier memory system

use crate::core::error::SentinelError;
use crate::core::traits::VectorStore;
use crate::core::types::{AgentId, CanonicalMessage, MessageId};
use crate::memory::medium_term::{ConversationSummary, MediumTermMemory};
use crate::memory::short_term::{SharedShortTermMemory, ShortTermMemory};
use anyhow::{Context, Result};
use chrono::Utc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::sync::watch;
use tokio::time::interval;
use tracing::{error, info, warn};

/// Default check interval for the dreamer loop (30 seconds)
pub const DEFAULT_CHECK_INTERVAL: Duration = Duration::from_secs(30);

/// Default medium-term consolidation threshold (10 summaries)
pub const DEFAULT_MEDIUM_TERM_THRESHOLD: usize = 10;

/// Memory manager coordinating all three tiers of memory
pub struct MemoryManager {
    /// Short-term memory instances per agent (thread-safe)
    short_term_stores: Arc<RwLock<HashMap<AgentId, SharedShortTermMemory>>>,
    /// Medium-term memory (shared across all agents)
    medium_term: MediumTermMemory,
    /// Long-term memory (shared across all agents)
    long_term: Arc<dyn VectorStore>,
    /// Check interval for consolidation checks
    check_interval: Duration,
    /// Medium-term consolidation threshold
    medium_term_threshold: usize,
}

impl MemoryManager {
    /// Create a new memory manager
    ///
    /// # Arguments
    /// * `medium_term_path` - Path to the Sled database for medium-term memory
    /// * `long_term` - Vector store for long-term memory
    ///
    /// # Returns
    /// * `Ok(MemoryManager)` - Successfully created
    /// * `Err(anyhow::Error)` - Error if creation fails
    pub fn new<P: AsRef<Path>>(medium_term_path: P, long_term: Arc<dyn VectorStore>) -> Result<Self> {
        let medium_term = MediumTermMemory::new(medium_term_path)
            .context("Failed to create medium-term memory")?;

        Ok(Self {
            short_term_stores: Arc::new(RwLock::new(HashMap::new())),
            medium_term,
            long_term,
            check_interval: DEFAULT_CHECK_INTERVAL,
            medium_term_threshold: DEFAULT_MEDIUM_TERM_THRESHOLD,
        })
    }

    /// Create a new memory manager with custom settings
    ///
    /// # Arguments
    /// * `medium_term_path` - Path to the Sled database
    /// * `long_term` - Vector store for long-term memory
    /// * `check_interval` - Interval between consolidation checks
    /// * `medium_term_threshold` - Number of summaries before medium→long consolidation
    ///
    /// # Returns
    /// * `Ok(MemoryManager)` - Successfully created
    /// * `Err(anyhow::Error)` - Error if creation fails
    pub fn with_settings<P: AsRef<Path>>(
        medium_term_path: P,
        long_term: Arc<dyn VectorStore>,
        check_interval: Duration,
        medium_term_threshold: usize,
    ) -> Result<Self> {
        let medium_term = MediumTermMemory::new(medium_term_path)
            .context("Failed to create medium-term memory")?;

        Ok(Self {
            short_term_stores: Arc::new(RwLock::new(HashMap::new())),
            medium_term,
            long_term,
            check_interval,
            medium_term_threshold,
        })
    }

    /// Get or create short-term memory for an agent
    ///
    /// # Arguments
    /// * `agent_id` - The agent ID
    ///
    /// # Returns
    /// Shared short-term memory instance
    pub async fn get_short_term(&self, agent_id: AgentId) -> SharedShortTermMemory {
        let stores = self.short_term_stores.read().await;
        if let Some(memory) = stores.get(&agent_id) {
            return memory.clone();
        }
        drop(stores);

        // Create new short-term memory for this agent
        let mut stores = self.short_term_stores.write().await;
        let memory = Arc::new(RwLock::new(ShortTermMemory::new()));
        stores.insert(agent_id, memory.clone());
        memory
    }

    /// Check if short-term memory should be consolidated
    ///
    /// # Arguments
    /// * `agent_id` - The agent ID
    ///
    /// # Returns
    /// `true` if consolidation is needed
    pub async fn should_consolidate_short(&self, agent_id: AgentId) -> bool {
        let memory = self.get_short_term(agent_id).await;
        let guard = memory.read().await;
        guard.should_consolidate()
    }

    /// Check if medium-term memory should be consolidated
    ///
    /// # Arguments
    /// * `agent_id` - The agent ID
    ///
    /// # Returns
    /// `true` if consolidation is needed
    pub async fn should_consolidate_medium(&self, agent_id: AgentId) -> bool {
        match self.medium_term.list_summaries(agent_id) {
            Ok(summaries) => summaries.len() >= self.medium_term_threshold,
            Err(e) => {
                warn!("Failed to list summaries for agent {}: {}", agent_id, e);
                false
            }
        }
    }

    /// Consolidate short-term memory to medium-term memory
    ///
    /// # Arguments
    /// * `agent_id` - The agent ID
    ///
    /// # Returns
    /// * `Ok(())` - Successfully consolidated
    /// * `Err(anyhow::Error)` - Error during consolidation
    pub async fn consolidate_short_to_medium(&self, agent_id: AgentId) -> Result<()> {
        let memory = self.get_short_term(agent_id).await;
        let messages = {
            let mut guard = memory.write().await;
            let msgs = guard.get_messages();
            guard.clear().context("Failed to clear short-term memory")?;
            msgs
        };

        if messages.is_empty() {
            return Ok(());
        }

        // Generate simple summary (concatenation for now)
        let summary_text = generate_summary(&messages);
        let conversation_id = uuid::Uuid::new_v4().to_string();
        let message_count = messages.len() as u64;

        let summary = ConversationSummary::new(
            agent_id,
            conversation_id,
            summary_text,
            message_count,
        );

        self.medium_term
            .store_summary(summary)
            .context("Failed to store summary in medium-term memory")?;

        info!(
            "Consolidated {} messages from short-term to medium-term for agent {}",
            message_count, agent_id
        );

        Ok(())
    }

    /// Consolidate medium-term memory to long-term memory
    ///
    /// # Arguments
    /// * `agent_id` - The agent ID
    ///
    /// # Returns
    /// * `Ok(())` - Successfully consolidated
    /// * `Err(anyhow::Error)` - Error during consolidation
    pub async fn consolidate_medium_to_long(&self, agent_id: AgentId) -> Result<()> {
        let summaries = self
            .medium_term
            .list_summaries(agent_id)
            .context("Failed to list summaries")?;

        if summaries.is_empty() {
            return Ok(());
        }

        // For now: Simple placeholder - would need embedding generation
        // TODO: Generate embeddings for summaries and store in long-term
        warn!(
            "Medium-to-long consolidation not fully implemented (needs embedding generation) for agent {}",
            agent_id
        );

        // Note: In a full implementation, we would:
        // 1. Generate embeddings for each summary
        // 2. Store embeddings in long-term memory (Qdrant)
        // 3. Optionally delete from medium-term after successful storage

        Ok(())
    }

    /// Run the dreamer loop (background consolidation task)
    ///
    /// # Arguments
    /// * `shutdown_rx` - Shutdown signal receiver
    ///
    /// # Returns
    /// * `Ok(())` - Graceful shutdown
    /// * `Err(anyhow::Error)` - Error during operation
    pub async fn run_dreamer_loop(&self, mut shutdown_rx: watch::Receiver<()>) -> Result<()> {
        let mut check_interval = interval(self.check_interval);

        info!("Dreamer loop started (check interval: {:?})", self.check_interval);

        loop {
            tokio::select! {
                _ = check_interval.tick() => {
                    // Get all agent IDs with short-term memory
                    let agent_ids: Vec<AgentId> = {
                        let stores = self.short_term_stores.read().await;
                        stores.keys().copied().collect()
                    };

                    // Check each agent's memory
                    for agent_id in agent_ids {
                        // Check short-term consolidation
                        if self.should_consolidate_short(agent_id).await {
                            if let Err(e) = self.consolidate_short_to_medium(agent_id).await {
                                error!("Failed to consolidate short-to-medium for agent {}: {}", agent_id, e);
                            }
                        }

                        // Check medium-term consolidation
                        if self.should_consolidate_medium(agent_id).await {
                            if let Err(e) = self.consolidate_medium_to_long(agent_id).await {
                                error!("Failed to consolidate medium-to-long for agent {}: {}", agent_id, e);
                            }
                        }
                    }
                }
                _ = shutdown_rx.changed() => {
                    info!("Dreamer loop received shutdown signal");
                    break;
                }
            }
        }

        info!("Dreamer loop stopped");
        Ok(())
    }
}

/// Generate a simple summary from messages (concatenation for now)
///
/// # Arguments
/// * `messages` - Messages to summarize
///
/// # Returns
/// Summary string
fn generate_summary(messages: &[CanonicalMessage]) -> String {
    // Simple implementation: concatenate all message content
    // Future: Use LLM for intelligent summarization
    messages
        .iter()
        .map(|msg| format!("{:?}: {}", msg.role, msg.content))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Role;
    use std::sync::Arc;
    use tempfile::TempDir;

    // Mock vector store for testing
    struct MockVectorStore;

    #[async_trait::async_trait]
    impl VectorStore for MockVectorStore {
        async fn upsert(
            &self,
            _id: MessageId,
            _embedding: Vec<f32>,
            _metadata: HashMap<String, String>,
        ) -> Result<(), SentinelError> {
            Ok(())
        }

        async fn search(
            &self,
            _query_embedding: Vec<f32>,
            _limit: usize,
        ) -> Result<Vec<MessageId>, SentinelError> {
            Ok(Vec::new())
        }
    }

    #[tokio::test]
    async fn test_memory_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("sled_test");
        let long_term: Arc<dyn VectorStore> = Arc::new(MockVectorStore);

        let manager = MemoryManager::new(path, long_term);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_get_short_term_creates_if_missing() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("sled_test");
        let long_term: Arc<dyn VectorStore> = Arc::new(MockVectorStore);

        let manager = MemoryManager::new(path, long_term).unwrap();
        let agent_id = AgentId::new();

        let memory = manager.get_short_term(agent_id).await;
        let guard = memory.read().await;
        assert_eq!(guard.message_count(), 0);
    }

    #[tokio::test]
    async fn test_should_consolidate_short() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("sled_test");
        let long_term: Arc<dyn VectorStore> = Arc::new(MockVectorStore);

        let manager = MemoryManager::new(path, long_term).unwrap();
        let agent_id = AgentId::new();

        // Initially should not need consolidation
        assert!(!manager.should_consolidate_short(agent_id).await);

        // Add many messages to exceed threshold
        let memory = manager.get_short_term(agent_id).await;
        {
            let mut guard = memory.write().await;
            for _ in 0..200 {
                // Create a large message to exceed token threshold
                let large_content = "x".repeat(1000); // ~250 tokens each
                let msg = CanonicalMessage::new(Role::User, large_content);
                let _ = guard.append_message(msg);
            }
        }

        // Should now need consolidation
        assert!(manager.should_consolidate_short(agent_id).await);
    }

    #[tokio::test]
    async fn test_consolidate_short_to_medium() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("sled_test");
        let long_term: Arc<dyn VectorStore> = Arc::new(MockVectorStore);

        let manager = MemoryManager::new(path, long_term).unwrap();
        let agent_id = AgentId::new();

        // Add messages
        let memory = manager.get_short_term(agent_id).await;
        {
            let mut guard = memory.write().await;
            for i in 0..5 {
                let msg = CanonicalMessage::new(Role::User, format!("Message {}", i));
                let _ = guard.append_message(msg);
            }
        }

        // Consolidate
        let result = manager.consolidate_short_to_medium(agent_id).await;
        assert!(result.is_ok());

        // Short-term memory should be cleared
        let memory = manager.get_short_term(agent_id).await;
        let guard = memory.read().await;
        assert_eq!(guard.message_count(), 0);

        // Summary should be stored in medium-term
        let summaries = manager.medium_term.list_summaries(agent_id).unwrap();
        assert!(!summaries.is_empty());
    }
}
