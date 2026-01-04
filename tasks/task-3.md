# Task 3: Explicit State Machine Implementation

## Overview

Implement the explicit state machine for agent state transitions. This defines the valid state transitions and validation logic that the orchestrator will enforce.

## Dependencies

**REQUIRES:**
- ✅ **Task 2** (Channel-Based Communication) - Channel infrastructure in place
- ✅ **Phase 1** - `AgentState` enum defined in `src/core/types.rs`

## Objectives

1. Implement state transition validation logic
2. Create state machine transition functions
3. Add state transition error types
4. Implement state history tracking (optional, for debugging)

## Implementation Tasks

### 1. State Transition Logic

**Location**: `src/core/types.rs` (extend existing file) or `src/core/state_machine.rs` (new file)

**State Machine Definition** (from PRD):
```
Idle → Thinking → ToolCall → Reflecting → Idle
  ↑                                         │
  └─────────────────────────────────────────┘
```

**Requirements**:
- Function to validate state transitions: `validate_transition(from: AgentState, to: AgentState) -> Result<(), SentinelError>`
- Function to get valid next states: `valid_next_states(current: AgentState) -> Vec<AgentState>`
- State transition function: `transition_state(current: AgentState, next: AgentState) -> Result<AgentState, SentinelError>`

**Code Structure**:
```rust
impl AgentState {
    /// Validate if a state transition is allowed
    pub fn can_transition_to(&self, next: AgentState) -> bool {
        match (self, next) {
            (AgentState::Idle, AgentState::Thinking) => true,
            (AgentState::Thinking, AgentState::ToolCall) => true,
            (AgentState::Thinking, AgentState::Reflecting) => true,
            (AgentState::ToolCall, AgentState::Reflecting) => true,
            (AgentState::Reflecting, AgentState::Idle) => true,
            (AgentState::Idle, AgentState::Idle) => true, // Self-loop allowed
            _ => false,
        }
    }
    
    /// Get all valid next states from current state
    pub fn valid_next_states(&self) -> Vec<AgentState> {
        // ...
    }
}
```

### 2. State Transition Errors

**Location**: `src/core/error.rs`

**Requirements**:
- Add `InvalidStateTransition` error variant
- Include current and attempted states in error message
- Use `thiserror` for error derivation

**Error Definition**:
```rust
#[derive(thiserror::Error, Debug)]
pub enum SentinelError {
    // ... existing errors
    
    #[error("Invalid state transition: cannot transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: AgentState,
        to: AgentState,
    },
}
```

### 3. State Machine Module

**Location**: `src/core/state_machine.rs` (new file, optional)

**If creating separate module:**
- Export state transition functions
- Keep core types in `types.rs`
- Move transition logic to `state_machine.rs`

## Testing Requirements

### Unit Tests

**Location**: `src/core/types.rs` or `src/core/state_machine.rs`

**Test Cases**:
1. ✅ Valid transitions: Idle → Thinking → ToolCall → Reflecting → Idle
2. ✅ Invalid transitions: Idle → ToolCall (should fail)
3. ✅ Self-loops: Idle → Idle (should be allowed)
4. ✅ All valid next states returned correctly
5. ✅ Error messages include both states

**Test Pattern**:
```rust
#[test]
fn test_valid_state_transitions() {
    assert!(AgentState::Idle.can_transition_to(AgentState::Thinking));
    assert!(AgentState::Thinking.can_transition_to(AgentState::ToolCall));
    // ...
}

#[test]
fn test_invalid_state_transitions() {
    assert!(!AgentState::Idle.can_transition_to(AgentState::ToolCall));
    // ...
}
```

## Acceptance Criteria

- [ ] State transition validation logic implemented
- [ ] All valid transitions defined and tested
- [ ] Invalid transitions properly rejected with errors
- [ ] `InvalidStateTransition` error type added to `SentinelError`
- [ ] All tests pass: `cargo test core::state_machine` (or `core::types`)
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No external dependencies added to `src/core/` (maintains Phase 1 purity)

## Error Handling

- Use `thiserror` for domain errors (not `anyhow` in core)
- Return `Result<AgentState, SentinelError>` for transitions
- Never panic on invalid transitions - return errors

## References

- PRD Section: "State Machine Orchestration" (lines 43-54)
- PRD Section: "The Sentinel (Orchestrator)" (lines 58-68)
- Architecture Doc: "State Machine Orchestration" (lines 75-85)
- Start Guide: Phase 3, Work Item 1 (lines 108-127)

## Next Task

After completing this task, proceed to **task-4.md**: Actor Event Loops (The Sentinel)

