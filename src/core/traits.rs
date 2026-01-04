// Domain traits - LLMProvider, VectorStore, etc.
// These define the ports (interfaces) that adapters must implement.
// All traits use async-trait for async methods and must be mockable with mockall.

use crate::core::error::SentinelError;
use crate::core::types::{CanonicalMessage, MessageId};
use async_trait::async_trait;
use std::collections::HashMap;

/// Trait for LLM (Large Language Model) providers.
/// Implementations handle communication with LLM services (OpenAI, Anthropic, etc.)
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Complete a conversation with the LLM, returning a single response message.
    ///
    /// # Arguments
    /// * `messages` - Vector of canonical messages representing the conversation history
    ///
    /// # Returns
    /// * `Ok(CanonicalMessage)` - The LLM's response as a canonical message
    /// * `Err(SentinelError)` - Error if the completion fails
    async fn complete(
        &self,
        messages: Vec<CanonicalMessage>,
    ) -> Result<CanonicalMessage, SentinelError>;

    /// Stream a conversation with the LLM, returning chunks of the response.
    ///
    /// # Arguments
    /// * `messages` - Vector of canonical messages representing the conversation history
    ///
    /// # Returns
    /// * `Ok(Box<dyn Stream>)` - A boxed stream of string chunks from the LLM
    /// * `Err(SentinelError)` - Error if streaming fails
    ///
    /// # Note
    /// The stream yields `Result<String, SentinelError>` items, allowing per-chunk error handling.
    /// Returns a boxed stream to allow different implementations without associated types.
    async fn stream(
        &self,
        messages: Vec<CanonicalMessage>,
    ) -> Result<
        Box<dyn futures::Stream<Item = Result<String, SentinelError>> + Send + Unpin>,
        SentinelError,
    >;
}

/// Trait for vector storage (embedding databases like Qdrant).
/// Implementations handle storing and searching vector embeddings.
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Upsert (insert or update) a vector embedding with metadata.
    ///
    /// # Arguments
    /// * `id` - Message ID associated with this embedding
    /// * `embedding` - Vector of f32 values representing the embedding
    /// * `metadata` - Key-value pairs of metadata to store with the embedding
    ///
    /// # Returns
    /// * `Ok(())` - Successfully stored
    /// * `Err(SentinelError)` - Error if storage fails
    async fn upsert(
        &self,
        id: MessageId,
        embedding: Vec<f32>,
        metadata: HashMap<String, String>,
    ) -> Result<(), SentinelError>;

    /// Search for similar vectors using a query embedding.
    ///
    /// # Arguments
    /// * `query_embedding` - Vector of f32 values to search for
    /// * `limit` - Maximum number of results to return
    ///
    /// # Returns
    /// * `Ok(Vec<MessageId>)` - Vector of message IDs matching the query, ordered by similarity
    /// * `Err(SentinelError)` - Error if search fails
    async fn search(
        &self,
        query_embedding: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<MessageId>, SentinelError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{MessageId, Role};
    use mockall::mock;
    use mockall::predicate::*;

    // Mock LLMProvider trait
    mock! {
        pub LLMProvider {}

        #[async_trait]
        impl LLMProvider for LLMProvider {
            async fn complete(
                &self,
                messages: Vec<CanonicalMessage>,
            ) -> Result<CanonicalMessage, SentinelError>;

            async fn stream(
                &self,
                messages: Vec<CanonicalMessage>,
            ) -> Result<Box<dyn futures::Stream<Item = Result<String, SentinelError>> + Send + Unpin>, SentinelError>;
        }
    }

    // Mock VectorStore trait
    mock! {
        pub VectorStore {}

        #[async_trait]
        impl VectorStore for VectorStore {
            async fn upsert(
                &self,
                id: MessageId,
                embedding: Vec<f32>,
                metadata: HashMap<String, String>,
            ) -> Result<(), SentinelError>;

            async fn search(
                &self,
                query_embedding: Vec<f32>,
                limit: usize,
            ) -> Result<Vec<MessageId>, SentinelError>;
        }
    }

    #[tokio::test]
    async fn test_llm_provider_complete() {
        let mut mock_llm = MockLLMProvider::new();
        let test_message = CanonicalMessage::new(Role::User, "Hello".to_string());
        let expected_response = CanonicalMessage::new(Role::Assistant, "Hi there!".to_string());

        mock_llm
            .expect_complete()
            .withf(|msgs| msgs.len() == 1 && msgs[0].content == "Hello")
            .times(1)
            .returning(move |_| Ok(expected_response.clone()));

        let result = mock_llm.complete(vec![test_message]).await.unwrap();

        assert_eq!(result.role, Role::Assistant);
        assert_eq!(result.content, "Hi there!");
    }

    #[tokio::test]
    async fn test_vector_store_upsert() {
        let mut mock_store = MockVectorStore::new();
        let message_id = MessageId::new();
        let embedding = vec![0.1, 0.2, 0.3];
        let metadata = HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]);

        mock_store
            .expect_upsert()
            .with(eq(message_id), eq(embedding.clone()), eq(metadata.clone()))
            .times(1)
            .returning(|_, _, _| Ok(()));

        let result = mock_store.upsert(message_id, embedding, metadata).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_vector_store_search() {
        let mut mock_store = MockVectorStore::new();
        let query_embedding = vec![0.1, 0.2, 0.3];
        let limit = 5;
        let expected_ids = vec![MessageId::new(), MessageId::new()];

        mock_store
            .expect_search()
            .with(eq(query_embedding.clone()), eq(limit))
            .times(1)
            .returning(move |_, _| Ok(expected_ids.clone()));

        let result = mock_store.search(query_embedding, limit).await.unwrap();

        assert_eq!(result.len(), 2);
    }
}
