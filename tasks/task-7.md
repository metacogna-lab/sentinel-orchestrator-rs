# Task 7: Medium-Term Memory (Sled Integration)

## Overview

Implement the medium-term memory system using Sled embedded database. This stores summarized conversations that survive process restarts and serves as the bridge between short-term and long-term memory.

## Dependencies

**REQUIRES:**
- ✅ **Task 6** (Short-Term Memory) - In-memory conversation history
- ✅ **Phase 1** - Core types and traits defined
- ✅ **Phase 2** - Actor system for coordination

## Objectives

1. Create Sled adapter for medium-term memory storage
2. Implement conversation summary storage and retrieval
3. Add serialization/deserialization for `CanonicalMessage` summaries
4. Integrate with memory consolidation pipeline

## Implementation Tasks

### 1. Medium-Term Memory Structure

**Location**: `src/memory/medium_term.rs` (new file)

**Requirements**:
- Use Sled database for persistent storage
- Store conversation summaries (not full messages)
- Key-value structure: `agent_id:conversation_id -> summary_data`
- Thread-safe access

**Code Structure**:
```rust
pub struct MediumTermMemory {
    db: sled::Db,
    path: PathBuf,
}

pub struct ConversationSummary {
    pub agent_id: AgentId,
    pub conversation_id: String,
    pub summary: String,
    pub message_count: u64,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
```

### 2. Storage Operations

**Functions to Implement**:

**`store_summary(summary: ConversationSummary) -> Result<(), SentinelError>`**
- Serialize summary to bytes
- Store in Sled with composite key
- Handle serialization errors

**`get_summary(agent_id: AgentId, conversation_id: &str) -> Result<Option<ConversationSummary>, SentinelError>`**
- Retrieve summary from Sled
- Deserialize bytes to `ConversationSummary`
- Return `None` if not found

**`list_summaries(agent_id: AgentId) -> Result<Vec<ConversationSummary>, SentinelError>`**
- List all summaries for an agent
- Iterate over Sled keys with prefix
- Deserialize all matching summaries

**`delete_summary(agent_id: AgentId, conversation_id: &str) -> Result<(), SentinelError>`**
- Remove summary from database
- Handle missing key gracefully

### 3. Serialization

**Requirements**:
- Use `serde` with `bincode` or `rmp_serde` for binary serialization
- Efficient storage format
- Handle versioning (for future schema changes)
- Error handling for corrupted data

**Serialization Strategy**:
```rust
impl ConversationSummary {
    fn to_bytes(&self) -> Result<Vec<u8>, SentinelError> {
        bincode::serialize(self)
            .map_err(|e| SentinelError::InvalidMessage { reason: e.to_string() })
    }
    
    fn from_bytes(data: &[u8]) -> Result<Self, SentinelError> {
        bincode::deserialize(data)
            .map_err(|e| SentinelError::InvalidMessage { reason: e.to_string() })
    }
}
```

### 4. Key Structure

**Key Format**: `{agent_id}:{conversation_id}`

**Requirements**:
- Unique keys per agent-conversation pair
- Efficient prefix scanning for agent summaries
- Human-readable for debugging

## Testing Requirements

### Unit Tests

**Location**: `src/memory/medium_term.rs`

**Test Cases**:
1. ✅ Store and retrieve summary
2. ✅ List summaries for agent
3. ✅ Delete summary
4. ✅ Handle missing summary gracefully
5. ✅ Serialization/deserialization round-trip
6. ✅ Multiple agents don't interfere

**Test Pattern**:
```rust
#[tokio::test]
async fn test_store_and_retrieve() {
    let temp_dir = tempfile::tempdir().unwrap();
    let mut memory = MediumTermMemory::new(temp_dir.path()).unwrap();
    
    let summary = ConversationSummary { /* ... */ };
    memory.store_summary(summary.clone()).unwrap();
    
    let retrieved = memory.get_summary(summary.agent_id, &summary.conversation_id).unwrap();
    assert!(retrieved.is_some());
}
```

### Integration Tests

**Location**: `tests/memory_integration.rs` (optional)

**Test Cases**:
- Persistence across process restarts
- Concurrent access
- Database corruption handling

## Acceptance Criteria

- [ ] Medium-term memory struct defined with Sled integration
- [ ] Store/retrieve/list/delete operations implemented
- [ ] Serialization working correctly
- [ ] All tests pass: `cargo test memory::medium_term`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] Database path configurable
- [ ] Error handling for corrupted data

## Error Handling

- Use `thiserror` for domain errors
- Convert Sled errors to `SentinelError`
- Handle serialization errors gracefully
- Never panic on database errors

## Performance Considerations

- Use efficient serialization format (bincode recommended)
- Batch operations when possible
- Consider Sled's transaction support for atomic operations

## References

- PRD Section: "Memory System" (lines 94-108)
- PRD Section: "Three-Tier Memory Consolidation" (lines 96-108)
- Architecture Doc: "Memory Management" (lines 162-179)
- Sled Documentation: https://docs.rs/sled/

## Next Task

After completing this task, proceed to **task-8.md**: Long-Term Memory (Qdrant Integration)

