# Task 9: Memory Manager (The Dreamer)

## Overview

Implement The Dreamer - the background memory manager that coordinates the three-tier memory system. This component monitors token counts, triggers consolidation, and manages the flow from short-term → medium-term → long-term memory.

## Dependencies

**REQUIRES:**
- ✅ **Task 6** (Short-Term Memory) - In-memory conversation history
- ✅ **Task 7** (Medium-Term Memory) - Sled integration
- ✅ **Task 8** (Long-Term Memory) - Qdrant integration
- ✅ **Phase 2** - Actor system for coordination

## Objectives

1. Implement memory manager coordinating all three tiers
2. Create background task (The Dreamer) for periodic monitoring
3. Implement consolidation logic (short → medium → long)
4. Add token counting and threshold detection

## Implementation Tasks

### 1. Memory Manager Structure

**Location**: `src/memory/manager.rs` (extend existing)

**Requirements**:
- Coordinate short-term, medium-term, and long-term memory
- Track token counts across tiers
- Manage consolidation triggers
- Background task for periodic checks

**Code Structure**:
```rust
pub struct MemoryManager {
    short_term: Arc<RwLock<ShortTermMemory>>,
    medium_term: MediumTermMemory,
    long_term: Arc<dyn VectorStore>,
    token_threshold: u64,
    check_interval: Duration,
}
```

### 2. Consolidation Logic

**Functions to Implement**:

**`consolidate_short_to_medium(agent_id: AgentId) -> Result<(), anyhow::Error>`**
- Check if short-term memory exceeds threshold
- Generate summary of conversation (using LLM if available, or simple concatenation for now)
- Store summary in medium-term memory
- Clear short-term memory after successful storage

**`consolidate_medium_to_long(agent_id: AgentId) -> Result<(), anyhow::Error>`**
- Retrieve summaries from medium-term memory
- Generate embeddings for summaries (if embedding generator available)
- Store embeddings in long-term memory (Qdrant)
- Optionally: Archive or delete from medium-term after consolidation

**`should_consolidate_short() -> bool`**
- Check if short-term token count exceeds threshold
- Return true if consolidation needed

**`should_consolidate_medium() -> bool`**
- Check if medium-term has summaries ready for long-term
- Could be based on count or age

### 3. Background Task (The Dreamer)

**Function**: `run_dreamer_loop(mut manager: MemoryManager, shutdown_rx: watch::Receiver<()>) -> Result<(), anyhow::Error>`

**Requirements**:
- Periodic checks (default: every 30 seconds)
- Monitor token counts
- Trigger consolidation when thresholds exceeded
- Handle errors gracefully (log, don't crash)
- Support graceful shutdown

**Event Loop Pattern**:
```rust
async fn run_dreamer_loop(
    mut manager: MemoryManager,
    mut shutdown_rx: watch::Receiver<()>,
) -> Result<(), anyhow::Error> {
    let mut interval = tokio::time::interval(manager.check_interval);
    
    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Check consolidation triggers
                if manager.should_consolidate_short() {
                    if let Err(e) = manager.consolidate_short_to_medium(agent_id).await {
                        error!("Consolidation error: {}", e);
                    }
                }
                
                if manager.should_consolidate_medium() {
                    if let Err(e) = manager.consolidate_medium_to_long(agent_id).await {
                        error!("Consolidation error: {}", e);
                    }
                }
            }
            _ = shutdown_rx.changed() => {
                break;
            }
        }
    }
    
    Ok(())
}
```

### 4. Summary Generation

**Requirements**:
- For now: Simple concatenation or truncation
- Future: Integrate with LLM for intelligent summarization
- Generate `ConversationSummary` from `Vec<CanonicalMessage>`

**Simple Summary Strategy** (initial implementation):
```rust
fn generate_summary(messages: &[CanonicalMessage]) -> String {
    // Simple: concatenate first and last N messages
    // Or: truncate to key points
    // Future: LLM-based summarization
    messages.iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}
```

### 5. Integration with Actors

**Requirements**:
- Memory manager accessible to actors
- Actors can append messages to short-term memory
- Actors can query long-term memory for relevant context
- Thread-safe access patterns

## Testing Requirements

### Unit Tests

**Location**: `src/memory/manager.rs`

**Test Cases**:
1. ✅ Memory manager initializes correctly
2. ✅ Consolidation triggers when threshold exceeded
3. ✅ Short-to-medium consolidation works
4. ✅ Medium-to-long consolidation works
5. ✅ Background task runs and checks periodically
6. ✅ Graceful shutdown works
7. ✅ Error handling doesn't crash loop

**Test Pattern**:
```rust
#[tokio::test]
async fn test_consolidation_trigger() {
    let mut manager = MemoryManager::new(/* ... */);
    
    // Fill short-term memory beyond threshold
    for _ in 0..100 {
        manager.append_message(/* ... */).await?;
    }
    
    assert!(manager.should_consolidate_short());
    
    manager.consolidate_short_to_medium(agent_id).await?;
    assert!(!manager.should_consolidate_short());
}
```

## Acceptance Criteria

- [ ] Memory manager coordinates all three tiers
- [ ] Consolidation logic implemented
- [ ] Background task (Dreamer) runs correctly
- [ ] Token threshold detection works
- [ ] All tests pass: `cargo test memory::manager`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No `unwrap()` or `expect()` in production code

## Error Handling

- Use `anyhow::Result` for manager operations (application layer)
- Log all consolidation errors
- Never crash the background loop
- Retry logic for transient failures (optional)

## Configuration

**Default Values**:
- Token threshold: 50,000 tokens
- Check interval: 30 seconds
- Medium-term consolidation: Every 10 summaries or 24 hours

## References

- PRD Section: "The Dreamer (Memory Manager)" (lines 82-92)
- PRD Section: "Memory System" (lines 94-108)
- Architecture Doc: "Memory Management" (lines 162-179)
- Architecture Doc: "The Dreamer" (lines 219-229)

## Next Task

After completing this task, proceed to **task-10.md**: Token Counting and Consolidation Triggers

