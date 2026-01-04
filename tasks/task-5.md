# Task 5: Supervisor Actor

## Overview

Implement the Supervisor actor that manages agent lifecycle, monitors health, detects zombie processes (stuck >60s), and handles restarts.

## Dependencies

**REQUIRES:**
- ✅ **Task 2** (Channel-Based Communication) - Channel infrastructure
- ✅ **Task 3** (State Machine) - State transition validation
- ✅ **Task 4** (Actor Event Loops) - Base actor implementation

## Objectives

1. Implement supervisor actor for agent lifecycle management
2. Add zombie detection (agents stuck >60 seconds)
3. Implement health monitoring
4. Add agent restart capabilities

## Implementation Tasks

### 1. Supervisor Structure

**Location**: `src/engine/supervisor.rs`

**Supervisor Definition**:
```rust
pub struct Supervisor {
    pub agents: HashMap<AgentId, AgentHandle>,
    pub health_check_interval: Duration,
    pub zombie_timeout: Duration, // Default: 60 seconds
}

pub struct AgentHandle {
    pub tx: mpsc::Sender<ActorMessage>,
    pub handle: tokio::task::JoinHandle<Result<(), anyhow::Error>>,
    pub last_activity: DateTime<Utc>,
    pub state: AgentState,
}
```

**Requirements**:
- Track all spawned agents
- Monitor last activity timestamp
- Store agent communication channels
- Track agent state

### 2. Agent Lifecycle Management

**Location**: `src/engine/supervisor.rs`

**Functions to Implement**:

**`spawn_agent() -> Result<AgentId, anyhow::Error>`**
- Create new actor using Task 4's `spawn_actor()`
- Register agent in supervisor's tracking map
- Return `AgentId` for the new agent
- Initialize `last_activity` to current time

**`terminate_agent(id: AgentId) -> Result<(), anyhow::Error>`**
- Close agent's channel (drop sender)
- Wait for agent task to complete
- Remove agent from tracking map
- Handle errors gracefully

**`restart_agent(id: AgentId) -> Result<AgentId, anyhow::Error>`**
- Terminate existing agent
- Spawn new agent
- Return new `AgentId`

### 3. Health Monitoring

**Location**: `src/engine/supervisor.rs`

**Function**: `check_agent_health(id: AgentId) -> Result<AgentHealth, anyhow::Error>`

**AgentHealth Type**:
```rust
pub struct AgentHealth {
    pub id: AgentId,
    pub state: AgentState,
    pub last_activity: DateTime<Utc>,
    pub is_alive: bool,
    pub is_zombie: bool,
}
```

**Requirements**:
- Check if agent task is still running
- Verify last activity timestamp
- Determine if agent is zombie (stuck >60s)
- Return health status

### 4. Zombie Detection

**Location**: `src/engine/supervisor.rs`

**Function**: `detect_zombies() -> Vec<AgentId>`

**Requirements**:
- Iterate through all tracked agents
- Check if `last_activity` is older than `zombie_timeout` (60s)
- Check if agent task is still running but unresponsive
- Return list of zombie agent IDs

**Detection Logic**:
```rust
fn is_zombie(handle: &AgentHandle) -> bool {
    let time_since_activity = Utc::now() - handle.last_activity;
    time_since_activity > self.zombie_timeout && handle.handle.is_finished() == false
}
```

### 5. Supervisor Event Loop

**Location**: `src/engine/supervisor.rs`

**Event Loop Pattern**:
```rust
async fn supervisor_loop(mut supervisor: Supervisor) -> Result<(), anyhow::Error> {
    let mut health_check_interval = tokio::time::interval(supervisor.health_check_interval);
    
    loop {
        tokio::select! {
            // Health check tick
            _ = health_check_interval.tick() => {
                let zombies = supervisor.detect_zombies();
                for zombie_id in zombies {
                    supervisor.terminate_agent(zombie_id)?;
                    // Optionally restart
                }
            }
            // Future: spawn requests, termination requests, etc.
            _ = shutdown_rx.changed() => {
                // Graceful shutdown: terminate all agents
                break;
            }
        }
    }
    
    Ok(())
}
```

**Requirements**:
- Periodic health checks (default: every 10 seconds)
- Zombie detection and cleanup
- Graceful shutdown of all agents
- Handle errors without crashing

### 6. Activity Tracking

**Requirements**:
- Update `last_activity` when agent processes messages
- This requires coordination with Task 4 actor implementation
- Consider adding activity ping mechanism

**Option A**: Actor reports activity to supervisor via channel
**Option B**: Supervisor tracks based on message send/receive
**Option C**: Actor updates shared state (use channels, not `Arc<Mutex>`)

**Recommended**: Option A - actors send activity updates to supervisor

## Testing Requirements

### Unit Tests

**Location**: `src/engine/supervisor.rs`

**Test Cases**:
1. ✅ Supervisor spawns agents correctly
2. ✅ Supervisor tracks agent handles
3. ✅ Health check detects healthy agents
4. ✅ Zombie detection identifies stuck agents (>60s)
5. ✅ Terminate agent closes channel and cleans up
6. ✅ Restart agent creates new agent and removes old
7. ✅ Supervisor handles agent task panics gracefully
8. ✅ Graceful shutdown terminates all agents

**Test Pattern**:
```rust
#[tokio::test]
async fn test_zombie_detection() {
    let mut supervisor = Supervisor::new();
    let agent_id = supervisor.spawn_agent()?;
    
    // Simulate agent stuck (don't update last_activity)
    // Fast-forward time or wait
    tokio::time::sleep(Duration::from_secs(61)).await;
    
    let zombies = supervisor.detect_zombies();
    assert!(zombies.contains(&agent_id));
}
```

### Integration Tests

**Location**: `tests/supervisor_integration.rs` (optional)

**Test Cases**:
- Multiple agents managed by supervisor
- Concurrent agent spawning and termination
- Zombie cleanup in production-like scenario

## Acceptance Criteria

- [ ] Supervisor struct defined with agent tracking
- [ ] Agent lifecycle functions implemented (spawn, terminate, restart)
- [ ] Health monitoring functional
- [ ] Zombie detection works correctly (>60s timeout)
- [ ] Supervisor event loop runs with periodic health checks
- [ ] Graceful shutdown terminates all agents
- [ ] All tests pass: `cargo test engine::supervisor`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No `unwrap()` or `expect()` in production code

## Error Handling

- Use `anyhow::Result` for supervisor operations
- Log all agent lifecycle events with `tracing`
- Handle agent task failures gracefully
- Never panic - always return errors

## Performance Considerations

- Health check interval: 10 seconds (configurable)
- Zombie timeout: 60 seconds (from PRD)
- Efficient agent lookup using `HashMap<AgentId, AgentHandle>`
- Consider using `tokio::time::interval` for periodic checks

## References

- PRD Section: "The Supervisor" (lines 70-80)
- PRD Section: "Zombie Detection" (lines 134-137)
- Architecture Doc: "The Supervisor" (lines 207-217)
- Start Guide: Phase 3, Work Item 2 (lines 123-127)

## Phase 2 Completion

After completing this task, **Phase 2: Actor System** is complete. All components are in place:
- ✅ Channel-based communication (Task 2)
- ✅ Explicit state machine (Task 3)
- ✅ Actor event loops (Task 4)
- ✅ Supervisor actor (Task 5)

**Next Phase**: Proceed to **Phase 3: Memory System** (see PRD lines 202-205)

