# Task 8: Long-Term Memory (Qdrant Integration)

## Overview

Implement the long-term memory system using Qdrant vector database. This stores embeddings of conversation summaries for semantic search and retrieval, enabling agents to access relevant past conversations.

## Dependencies

**REQUIRES:**
- ✅ **Task 7** (Medium-Term Memory) - Sled integration for summaries
- ✅ **Phase 1** - `VectorStore` trait defined in `src/core/traits.rs`
- ✅ **Phase 2** - Actor system for coordination

## Objectives

1. Implement Qdrant adapter for vector storage
2. Create embedding generation integration point
3. Implement semantic search functionality
4. Store conversation summaries as embeddings

## Implementation Tasks

### 1. Qdrant Adapter Implementation

**Location**: `src/adapters/qdrant.rs` (extend existing or create)

**Requirements**:
- Implement `VectorStore` trait from `src/core/traits.rs`
- Connect to Qdrant instance (configurable URL)
- Create/use collection for embeddings
- Map domain types to Qdrant types

**Code Structure**:
```rust
pub struct QdrantStore {
    client: qdrant_client::QdrantClient,
    collection_name: String,
}

#[async_trait]
impl VectorStore for QdrantStore {
    async fn upsert(
        &self,
        id: MessageId,
        embedding: Vec<f32>,
        metadata: HashMap<String, String>,
    ) -> Result<(), SentinelError> {
        // Map MessageId to Qdrant point ID
        // Store embedding with metadata
    }
    
    async fn search(
        &self,
        query_embedding: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<MessageId>, SentinelError> {
        // Perform vector search
        // Return matching MessageIds
    }
}
```

### 2. Collection Management

**Requirements**:
- Create collection on initialization if not exists
- Configure vector size (from embedding model)
- Set distance metric (cosine similarity recommended)
- Handle collection already exists gracefully

**Collection Configuration**:
```rust
let collection_config = qdrant_client::qdrant::CreateCollection {
    collection_name: collection_name.clone(),
    vectors_config: Some(qdrant_client::qdrant::VectorsConfig {
        config: Some(qdrant_client::qdrant::VectorsConfigConfig::Params(
            qdrant_client::qdrant::VectorParams {
                size: embedding_dim as u64,
                distance: qdrant_client::qdrant::Distance::Cosine as i32,
            },
        )),
    }),
    ..Default::default()
};
```

### 3. Embedding Integration Point

**Requirements**:
- Define interface for embedding generation (trait or function)
- For now, accept pre-generated embeddings
- Future: Integrate with embedding service (OpenAI, local model, etc.)

**Embedding Interface**:
```rust
pub trait EmbeddingGenerator: Send + Sync {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, SentinelError>;
}
```

**Note**: Embedding generation can be implemented later. For now, accept embeddings as parameters.

### 4. Metadata Storage

**Requirements**:
- Store conversation metadata with embeddings
- Include: agent_id, conversation_id, timestamp, summary text
- Enable filtering by metadata in searches
- Map metadata to Qdrant payload

**Metadata Structure**:
```rust
let payload = serde_json::json!({
    "agent_id": agent_id.to_string(),
    "conversation_id": conversation_id,
    "timestamp": timestamp.to_rfc3339(),
    "summary": summary_text,
});
```

## Testing Requirements

### Unit Tests

**Location**: `src/adapters/qdrant.rs`

**Test Cases**:
1. ✅ Create collection on initialization
2. ✅ Upsert embedding with metadata
3. ✅ Search returns relevant results
4. ✅ Handle missing collection gracefully
5. ✅ Metadata filtering works
6. ✅ Error handling for connection failures

**Test Pattern**:
```rust
#[tokio::test]
async fn test_upsert_and_search() {
    let store = QdrantStore::new("http://localhost:6333", "test_collection").await?;
    
    let embedding = vec![0.1, 0.2, 0.3]; // Mock embedding
    let metadata = HashMap::from([("key".to_string(), "value".to_string())];
    let message_id = MessageId::new();
    
    store.upsert(message_id, embedding.clone(), metadata).await?;
    
    let results = store.search(embedding, 5).await?;
    assert!(results.contains(&message_id));
}
```

### Integration Tests

**Location**: `tests/qdrant_integration.rs` (optional)

**Test Cases**:
- Real Qdrant instance (Docker/Testcontainers)
- Semantic search accuracy
- Concurrent operations
- Collection management

## Acceptance Criteria

- [ ] Qdrant adapter implements `VectorStore` trait
- [ ] Collection creation/management works
- [ ] Upsert and search operations functional
- [ ] Metadata storage and retrieval works
- [ ] All tests pass: `cargo test adapters::qdrant`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] Connection URL configurable via environment/config

## Error Handling

- Use `anyhow` for adapter layer errors
- Convert Qdrant errors to `SentinelError`
- Handle connection failures gracefully
- Never panic on database errors

## Configuration

**Environment Variables**:
- `QDRANT_URL` - Qdrant server URL (default: `http://localhost:6333`)
- `QDRANT_COLLECTION` - Collection name (default: `sentinel_memories`)

## References

- PRD Section: "Memory System" (lines 94-108)
- PRD Section: "Vector Store Trait" (lines 181-183)
- Architecture Doc: "Memory Management" (lines 162-179)
- Qdrant Rust Client: https://docs.rs/qdrant-client/

## Next Task

After completing this task, proceed to **task-9.md**: Memory Manager (The Dreamer)

