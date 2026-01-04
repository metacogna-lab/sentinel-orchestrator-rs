# Task 2: Channel-Based Communication Infrastructure

## Overview

Implement the foundational channel-based communication system using `tokio::sync::mpsc` for actor message passing. This establishes the communication patterns that all actors will use.

## Dependencies

**REQUIRES Phase 1 (Core Domain) to be complete:**
- ✅ `CanonicalMessage` type defined in `src/core/types.rs`
- ✅ `AgentId` type defined in `src/core/types.rs`
- ✅ Core error types in `src/core/error.rs`

## Objectives

1. Create bounded channel infrastructure for actor communication
2. Define message channel types and helpers
3. Implement channel creation utilities
4. Add tests for channel behavior and backpressure

## Implementation Tasks

### 1. Channel Types and Utilities

**Location**: `src/engine/actor.rs` (or create `src/engine/channels.rs`)

**Requirements**:
- Define `ActorMessage` type wrapping `CanonicalMessage` with metadata
- Create channel creation helpers using bounded channels only
- Default channel size: 32 (configurable)
- Never use `unbounded_channel()` - always bounded

**Code Structure**:
```rust
// Channel message wrapper (may include sender info, priority, etc.)
pub struct ActorMessage {
    pub message: CanonicalMessage,
    pub sender: Option<AgentId>,
    // ... other metadata
}

// Channel creation helper
pub fn create_actor_channel(buffer_size: usize) -> (mpsc::Sender<ActorMessage>, mpsc::Receiver<ActorMessage>) {
    mpsc::channel(buffer_size)
}
```

### 2. Backpressure Handling

**Requirements**:
- All channels must be bounded
- Handle `send()` errors gracefully (channel full)
- Return appropriate errors when backpressure occurs
- Log backpressure events for observability

### 3. Channel Utilities Module

**Location**: `src/engine/channels.rs` (new file)

**Functions to implement**:
- `create_actor_channel(size: usize)` - Create bounded channel
- `try_send_with_timeout()` - Send with timeout handling
- Channel health checking utilities

## Testing Requirements

### Unit Tests

**Location**: `src/engine/channels.rs` (or `src/engine/actor.rs`)

**Test Cases**:
1. ✅ Channel creation with valid buffer size
2. ✅ Channel backpressure when buffer is full
3. ✅ Message ordering is preserved
4. ✅ Channel closure handling
5. ✅ Concurrent send/receive operations

**Test Pattern**:
```rust
#[tokio::test]
async fn test_channel_backpressure() {
    let (tx, mut rx) = create_actor_channel(2);
    
    // Fill channel
    tx.send(msg1).await?;
    tx.send(msg2).await?;
    
    // Next send should handle backpressure
    // ...
}
```

## Acceptance Criteria

- [ ] All channels use bounded `mpsc::channel(N)` (never unbounded)
- [ ] Channel utilities module created and tested
- [ ] Backpressure handling implemented and tested
- [ ] All tests pass: `cargo test engine::channels`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`

## Error Handling

- Use `anyhow::Result` for channel operations
- Convert channel errors to appropriate domain errors
- Log all backpressure events with `tracing`

## References

- PRD Section: "Actor Model with Message Passing" (lines 34-41)
- Architecture Doc: "Actor Model with Message Passing" (lines 56-74)
- Start Guide: Phase 3, Work Item 1 (lines 108-127)

## Next Task

After completing this task, proceed to **task-3.md**: Explicit State Machine Implementation

