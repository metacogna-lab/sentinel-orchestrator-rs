# Task 4: Actor Event Loops (The Sentinel)

## Overview

Implement the main actor event loop for The Sentinel (orchestrator). This is the core actor that manages state transitions, coordinates memory access, and executes LLM provider calls.

## Dependencies

**REQUIRES:**
- ✅ **Task 2** (Channel-Based Communication) - Channel infrastructure
- ✅ **Task 3** (State Machine) - State transition validation
- ✅ **Phase 1** - Core types, traits, and errors defined

## Objectives

1. Implement the main actor event loop with `tokio::select!`
2. Integrate state machine transitions
3. Set up message processing pipeline
4. Add cancellation and timeout handling

## Implementation Tasks

### 1. Actor Structure

**Location**: `src/engine/actor.rs`

**Actor Definition**:
```rust
pub struct Actor {
    pub id: AgentId,
    pub state: AgentState,
    pub rx: mpsc::Receiver<ActorMessage>,
    // Future: LLM provider, memory manager handles
}
```

**Requirements**:
- Actor owns its receiver channel
- Actor maintains current state
- Actor has unique `AgentId`

### 2. Main Event Loop

**Location**: `src/engine/actor.rs`

**Event Loop Pattern** (from Architecture Doc):
```rust
async fn actor_loop(rx: mpsc::Receiver<ActorMessage>) {
    let mut state = AgentState::Idle;
    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Some(msg) => {
                        state = process_message(msg, state).await?;
                    }
                    None => {
                        // Channel closed, exit loop
                        break;
                    }
                }
            }
            // Future: shutdown signal, timeout branches
        }
    }
}
```

**Requirements**:
- Use `tokio::select!` for cancellation safety
- Handle channel closure gracefully
- Process messages and update state
- Validate state transitions using Task 3 logic

### 3. Message Processing

**Location**: `src/engine/actor.rs`

**Function**: `process_message(msg: ActorMessage, current_state: AgentState) -> Result<AgentState, anyhow::Error>`

**Requirements**:
- Extract `CanonicalMessage` from `ActorMessage`
- Determine appropriate next state based on message
- Validate state transition before applying
- Return new state or error

**State Transition Logic**:
- On receiving message in `Idle` → transition to `Thinking`
- After processing → transition to `Reflecting`
- After reflection → transition back to `Idle`

### 4. Cancellation and Timeouts

**Requirements**:
- Support shutdown signal via channel or `tokio::sync::watch`
- Timeout detection for stuck states (future: >60s)
- Graceful shutdown on channel close

**Pattern**:
```rust
tokio::select! {
    msg = rx.recv() => { /* handle */ }
    _ = shutdown_rx.changed() => {
        // Graceful shutdown
        break;
    }
    _ = tokio::time::sleep(Duration::from_secs(60)) => {
        // Timeout handling (future: zombie detection)
    }
}
```

### 5. Actor Spawning

**Location**: `src/engine/actor.rs`

**Function**: `spawn_actor(buffer_size: usize) -> (mpsc::Sender<ActorMessage>, tokio::task::JoinHandle<Result<(), anyhow::Error>>)`

**Requirements**:
- Create bounded channel
- Spawn actor loop as tokio task
- Return sender and join handle
- Handle task errors appropriately

## Testing Requirements

### Unit Tests

**Location**: `src/engine/actor.rs`

**Test Cases**:
1. ✅ Actor spawns and receives messages
2. ✅ State transitions correctly on message receipt
3. ✅ Invalid state transitions are rejected
4. ✅ Channel closure causes graceful shutdown
5. ✅ Multiple messages processed in order
6. ✅ Actor handles backpressure correctly

**Test Pattern**:
```rust
#[tokio::test]
async fn test_actor_state_transitions() {
    let (tx, handle) = spawn_actor(32);
    
    let msg = ActorMessage {
        message: CanonicalMessage::new(Role::User, "test".to_string()),
        sender: None,
    };
    
    tx.send(msg).await?;
    
    // Verify state transition occurred
    // ...
    
    drop(tx); // Close channel
    handle.await?; // Wait for graceful shutdown
}
```

### Integration Tests

**Location**: `tests/actor_integration.rs` (optional)

**Test Cases**:
- Multiple actors running concurrently
- Message routing between actors
- State machine enforcement across actors

## Acceptance Criteria

- [ ] Actor struct defined with required fields
- [ ] Main event loop implemented with `tokio::select!`
- [ ] Message processing pipeline functional
- [ ] State transitions validated and applied
- [ ] Cancellation handling implemented
- [ ] Actor spawning function works correctly
- [ ] All tests pass: `cargo test engine::actor`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No `unwrap()` or `expect()` in production code

## Error Handling

- Use `anyhow::Result` for actor operations (application layer)
- Convert `SentinelError` from state machine to `anyhow::Error`
- Log all errors with `tracing`
- Never panic - always return errors

## Performance Considerations

- Use bounded channels (default: 32)
- Process messages sequentially (for now)
- Future: concurrent message processing if needed

## References

- PRD Section: "The Sentinel (Orchestrator)" (lines 58-68)
- PRD Section: "Actor Model with Message Passing" (lines 34-41)
- Architecture Doc: "Actor Communication" (lines 56-74)
- Architecture Doc: "The Sentinel" (lines 195-205)
- Start Guide: Phase 3, Work Item 1 (lines 108-127)

## Next Task

After completing this task, proceed to **task-5.md**: Supervisor Actor

