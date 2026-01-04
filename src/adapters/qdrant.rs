// Qdrant vector database adapter implementation
// Implements VectorStore trait for long-term memory storage

use crate::core::error::SentinelError;
use crate::core::traits::VectorStore;
use crate::core::types::MessageId;
use async_trait::async_trait;
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollection, Distance, PointStruct, ScoredPoint, SearchPoints,
    UpsertPoints, VectorParams, VectorsConfig,
};
use qdrant_client::Qdrant;
use std::collections::HashMap;
use std::env;
use tracing::{debug, info, warn};

/// Default Qdrant server URL
const DEFAULT_QDRANT_URL: &str = "http://localhost:6333";

/// Default collection name for long-term memories
const DEFAULT_COLLECTION_NAME: &str = "sentinel_memories";

/// Default vector dimension (1536 for OpenAI embeddings, 384 for sentence-transformers)
/// This should match the embedding model being used
const DEFAULT_VECTOR_DIM: u64 = 1536;

/// Qdrant vector store implementation
pub struct QdrantStore {
    client: Qdrant,
    collection_name: String,
    vector_dim: u64,
}

impl QdrantStore {
    /// Create a new Qdrant store with default settings
    ///
    /// # Returns
    /// * `Ok(QdrantStore)` - Successfully created
    /// * `Err(SentinelError)` - Error if connection fails
    pub async fn new() -> Result<Self, SentinelError> {
        let url = env::var("QDRANT_URL").unwrap_or_else(|_| DEFAULT_QDRANT_URL.to_string());
        let collection_name =
            env::var("QDRANT_COLLECTION").unwrap_or_else(|_| DEFAULT_COLLECTION_NAME.to_string());
        Self::with_config(&url, &collection_name, DEFAULT_VECTOR_DIM).await
    }

    /// Create a new Qdrant store with custom configuration
    ///
    /// # Arguments
    /// * `url` - Qdrant server URL
    /// * `collection_name` - Name of the collection to use/create
    /// * `vector_dim` - Dimension of the embedding vectors
    ///
    /// # Returns
    /// * `Ok(QdrantStore)` - Successfully created
    /// * `Err(SentinelError)` - Error if connection or collection creation fails
    pub async fn with_config(
        url: &str,
        collection_name: &str,
        vector_dim: u64,
    ) -> Result<Self, SentinelError> {
        let client = Qdrant::from_url(url)
            .build()
            .map_err(|e| SentinelError::DomainViolation {
                rule: format!("Failed to connect to Qdrant at {}: {}", url, e),
            })?;

        let store = Self {
            client,
            collection_name: collection_name.to_string(),
            vector_dim,
        };

        // Ensure collection exists
        store.ensure_collection().await?;

        info!(
            "Qdrant store initialized: collection={}, vector_dim={}",
            collection_name, vector_dim
        );

        Ok(store)
    }

    /// Ensure the collection exists, creating it if necessary
    ///
    /// # Returns
    /// * `Ok(())` - Collection exists or was created
    /// * `Err(SentinelError)` - Error if collection creation fails
    async fn ensure_collection(&self) -> Result<(), SentinelError> {
        // Check if collection exists
        match self.client.collection_info(&self.collection_name).await {
            Ok(_) => {
                debug!("Collection {} already exists", self.collection_name);
                return Ok(());
            }
            Err(e) => {
                // Collection doesn't exist or error - try to create
                debug!(
                    "Collection {} not found or error: {}, creating...",
                    self.collection_name, e
                );
            }
        }

        // Create collection
        let create_collection = CreateCollection {
            collection_name: self.collection_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: self.vector_dim,
                    distance: Distance::Cosine as i32,
                    ..Default::default()
                })),
            }),
            ..Default::default()
        };

        self.client
            .create_collection(create_collection)
            .await
            .map_err(|e| SentinelError::DomainViolation {
                rule: format!(
                    "Failed to create collection {}: {}",
                    self.collection_name, e
                ),
            })?;

        info!("Created Qdrant collection: {}", self.collection_name);
        Ok(())
    }

    /// Convert MessageId (UUID) to Qdrant point ID
    /// Qdrant supports UUID point IDs directly
    fn message_id_to_point_id(&self, id: MessageId) -> String {
        id.0.to_string()
    }

    /// Convert point ID string back to MessageId
    fn point_id_to_message_id(&self, point_id: &str) -> Result<MessageId, SentinelError> {
        uuid::Uuid::parse_str(point_id)
            .map(MessageId::from)
            .map_err(|e| SentinelError::InvalidMessage {
                reason: format!("Failed to parse point ID as UUID: {}", e),
            })
    }

    /// Convert metadata HashMap to Qdrant payload
    fn metadata_to_payload(
        &self,
        metadata: &HashMap<String, String>,
    ) -> HashMap<String, qdrant_client::qdrant::Value> {
        metadata
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    qdrant_client::qdrant::Value {
                        kind: Some(qdrant_client::qdrant::value::Kind::StringValue(v.clone())),
                    },
                )
            })
            .collect()
    }

    /// Extract UUID string from Qdrant PointId
    /// This handles both UUID and numeric point IDs
    fn extract_uuid_from_point_id(
        &self,
        point_id: &qdrant_client::qdrant::PointId,
    ) -> Result<String, SentinelError> {
        // PointId structure varies by Qdrant version
        // We stored the MessageId UUID as a string, so we need to extract it
        // Try to get the UUID string from the point ID
        // This is a simplified implementation - in production, handle all PointId variants
        if let Some(point_id_options) = &point_id.point_id_options {
            match point_id_options {
                qdrant_client::qdrant::point_id::PointIdOptions::Uuid(uuid_str) => {
                    Ok(uuid_str.clone())
                }
                qdrant_client::qdrant::point_id::PointIdOptions::Num(num) => {
                    // Convert numeric ID to UUID format (we stored as UUID, so this shouldn't happen)
                    // But handle it gracefully
                    Err(SentinelError::InvalidMessage {
                        reason: format!("Point ID is numeric ({}) but expected UUID", num),
                    })
                }
            }
        } else {
            Err(SentinelError::InvalidMessage {
                reason: "Point ID has no options".to_string(),
            })
        }
    }
}

#[async_trait]
impl VectorStore for QdrantStore {
    async fn upsert(
        &self,
        id: MessageId,
        embedding: Vec<f32>,
        metadata: HashMap<String, String>,
    ) -> Result<(), SentinelError> {
        // Validate embedding dimension
        if embedding.len() as u64 != self.vector_dim {
            return Err(SentinelError::InvalidMessage {
                reason: format!(
                    "Embedding dimension mismatch: expected {}, got {}",
                    self.vector_dim,
                    embedding.len()
                ),
            });
        }

        let point_id = self.message_id_to_point_id(id);
        let payload = self.metadata_to_payload(&metadata);

        let point = PointStruct::new(point_id, embedding, payload);

        let upsert_request = UpsertPoints {
            collection_name: self.collection_name.clone(),
            points: vec![point],
            ..Default::default()
        };

        self.client
            .upsert_points(upsert_request)
            .await
            .map_err(|e| SentinelError::DomainViolation {
                rule: format!("Failed to upsert point {}: {}", id, e),
            })?;

        debug!("Upserted embedding for message {}", id);
        Ok(())
    }

    async fn search(
        &self,
        query_embedding: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<MessageId>, SentinelError> {
        // Validate query embedding dimension
        if query_embedding.len() as u64 != self.vector_dim {
            return Err(SentinelError::InvalidMessage {
                reason: format!(
                    "Query embedding dimension mismatch: expected {}, got {}",
                    self.vector_dim,
                    query_embedding.len()
                ),
            });
        }

        let search_points = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query_embedding,
            limit: limit as u64,
            with_payload: Some(true.into()),
            ..Default::default()
        };

        let search_result = self
            .client
            .search_points(search_points)
            .await
            .map_err(|e| SentinelError::DomainViolation {
                rule: format!("Failed to search vectors: {}", e),
            })?;

        // Convert Qdrant point IDs back to MessageIds
        let message_ids: Vec<MessageId> = search_result
            .result
            .iter()
            .filter_map(|point: &ScoredPoint| {
                point.id.as_ref().and_then(|id| {
                    // Extract UUID from point ID
                    // Qdrant PointId can be UUID or num - we stored as UUID string
                    match self.extract_uuid_from_point_id(id) {
                        Ok(uuid_str) => self.point_id_to_message_id(&uuid_str).ok(),
                        Err(_) => {
                            warn!("Failed to extract UUID from point ID, skipping");
                            None
                        }
                    }
                })
            })
            .collect();

        let ids = message_ids;
        debug!("Search returned {} results", ids.len());
        Ok(ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Note: These tests require a running Qdrant instance
    // For unit tests, we'll test the logic without actual Qdrant connection
    // Integration tests should be in tests/qdrant_integration.rs

    #[test]
    fn test_metadata_to_payload() {
        let store = QdrantStore {
            client: Qdrant::from_url("http://localhost:6333").build().unwrap(),
            collection_name: "test".to_string(),
            vector_dim: 1536,
        };

        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());
        metadata.insert("key2".to_string(), "value2".to_string());

        let payload = store.metadata_to_payload(&metadata);
        assert_eq!(payload.len(), 2);
        assert!(payload.contains_key("key1"));
        assert!(payload.contains_key("key2"));
    }

    #[test]
    fn test_message_id_to_point_id() {
        let store = QdrantStore {
            client: Qdrant::from_url("http://localhost:6333").build().unwrap(),
            collection_name: "test".to_string(),
            vector_dim: 1536,
        };

        let message_id = MessageId::new();
        let point_id = store.message_id_to_point_id(message_id);
        assert!(!point_id.is_empty());
        assert!(uuid::Uuid::parse_str(&point_id).is_ok());
    }

    #[test]
    fn test_message_id_round_trip() {
        let store = QdrantStore {
            client: Qdrant::from_url("http://localhost:6333").build().unwrap(),
            collection_name: "test".to_string(),
            vector_dim: 1536,
        };

        let original_id = MessageId::new();
        let point_id = store.message_id_to_point_id(original_id);
        let recovered_id = store.point_id_to_message_id(&point_id).unwrap();

        assert_eq!(original_id, recovered_id);
    }

    #[test]
    fn test_point_id_to_message_id_invalid() {
        let store = QdrantStore {
            client: Qdrant::from_url("http://localhost:6333").build().unwrap(),
            collection_name: "test".to_string(),
            vector_dim: 1536,
        };

        let result = store.point_id_to_message_id("invalid-uuid");
        assert!(result.is_err());
    }

    // Integration test helper - requires Qdrant running
    #[tokio::test]
    #[ignore] // Ignore by default, run with --ignored flag
    async fn test_qdrant_integration() {
        let store = QdrantStore::with_config("http://localhost:6333", "test_collection", 3)
            .await
            .unwrap();

        let message_id = MessageId::new();
        let embedding = vec![0.1, 0.2, 0.3];
        let mut metadata = HashMap::new();
        metadata.insert("test_key".to_string(), "test_value".to_string());

        // Upsert
        store
            .upsert(message_id, embedding.clone(), metadata)
            .await
            .unwrap();

        // Search
        let results = store.search(embedding, 5).await.unwrap();
        assert!(results.contains(&message_id));
    }

    #[tokio::test]
    #[ignore]
    async fn test_embedding_dimension_validation() {
        let store = QdrantStore::with_config("http://localhost:6333", "test_collection", 3)
            .await
            .unwrap();

        let message_id = MessageId::new();
        let wrong_dim_embedding = vec![0.1, 0.2]; // Wrong dimension

        let result = store
            .upsert(message_id, wrong_dim_embedding, HashMap::new())
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            SentinelError::InvalidMessage { reason } => {
                assert!(reason.contains("dimension mismatch"));
            }
            _ => panic!("Expected InvalidMessage error"),
        }
    }
}
