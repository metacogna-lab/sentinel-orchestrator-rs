# Task 6: Short-Term Memory Implementation

## Overview

Implement the short-term memory system for in-memory conversation history. This is the first tier of the three-tier memory hierarchy and serves as the active conversation buffer.

## Dependencies

**REQUIRES:**
- ✅ **Phase 1** - `CanonicalMessage` type defined in `src/core/types.rs`
- ✅ **Phase 2** - Actor system with channels and state machine

## Objectives

1. Create in-memory conversation history storage
2. Implement message appending and retrieval
3. Add token counting functionality
4. Implement memory size limits and eviction policies

## Implementation Tasks

### 1. Short-Term Memory Structure

**Location**: `src/memory/short_term.rs` (new file)

**Requirements**:
- Store `Vec<CanonicalMessage>` for conversation history
- Track total token count (approximate)
- Implement size limits (configurable, default: 1000 messages or 100k tokens)
- Thread-safe access (use channels or `Arc<RwLock>` - prefer channels if possible)

**Code Structure**:
```rust
pub struct ShortTermMemory {
    messages: Vec<CanonicalMessage>,
    token_count: u64,
    max_messages: usize,
    max_tokens: u64,
}
```

### 2. Message Operations

**Functions to Implement**:

**`append_message(msg: CanonicalMessage) -> Result<(), SentinelError>`**
- Add message to history
- Update token count (approximate)
- Check if consolidation threshold reached
- Return error if memory full and eviction needed

**`get_messages() -> Vec<CanonicalMessage>`**
- Return all messages in order
- Used for LLM context

**`get_recent_messages(count: usize) -> Vec<CanonicalMessage>`**
- Return last N messages
- Useful for context window management

**`clear() -> Result<(), SentinelError>`**
- Clear all messages
- Reset token count
- Used after consolidation

### 3. Token Counting

**Requirements**:
- Approximate token counting (simple heuristic: chars / 4)
- Track cumulative token count
- Update on message append
- Provide `should_consolidate() -> bool` method

**Token Counting Strategy**:
- Simple approximation: `tokens ≈ characters / 4`
- Future: Can integrate with actual tokenizer if needed
- Threshold: Default 50k tokens (configurable)

### 4. Memory Limits and Eviction

**Requirements**:
- Enforce `max_messages` limit
- Enforce `max_tokens` limit
- When limit reached, trigger consolidation (don't evict, consolidate)
- Return error if consolidation needed but not possible

## Testing Requirements

### Unit Tests

**Location**: `src/memory/short_term.rs`

**Test Cases**:
1. ✅ Append messages and verify order
2. ✅ Token count updates correctly
3. ✅ `should_consolidate()` returns true when threshold exceeded
4. ✅ `get_recent_messages()` returns correct subset
5. ✅ `clear()` resets state correctly
6. ✅ Memory limits enforced

**Test Pattern**:
```rust
#[test]
fn test_append_message() {
    let mut memory = ShortTermMemory::new(100, 10000);
    let msg = CanonicalMessage::new(Role::User, "test".to_string());
    memory.append_message(msg).unwrap();
    assert_eq!(memory.message_count(), 1);
}
```

## Acceptance Criteria

- [ ] Short-term memory struct defined
- [ ] Message append/retrieval functions implemented
- [ ] Token counting functional
- [ ] Consolidation threshold detection works
- [ ] All tests pass: `cargo test memory::short_term`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No `unwrap()` or `expect()` in production code

## Error Handling

- Use `thiserror` for domain errors in core
- Return `Result<T, SentinelError>` for all operations
- Never panic on memory full - return error to trigger consolidation

## References

- PRD Section: "Memory System" (lines 94-108)
- PRD Section: "The Dreamer (Memory Manager)" (lines 82-92)
- Architecture Doc: "Memory Management" (lines 162-179)

## Next Task

After completing this task, proceed to **task-7.md**: Medium-Term Memory (Sled Integration)

