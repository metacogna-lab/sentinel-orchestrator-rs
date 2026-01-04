# Sentinel Orchestrator: Architecture Documentation

## Overview

The Sentinel Rust Orchestrator is a high-performance, multi-agent orchestration system built with strict adherence to Hexagonal Architecture principles. It coordinates AI agents through message-passing actors, manages multi-tier memory consolidation, and integrates with Cursor 2.0.

## Architectural Principles

### 1. Hexagonal Architecture (Ports & Adapters)

The project enforces strict boundaries between domain logic and infrastructure:

```
┌─────────────────────────────────────────┐
│           src/core/ (Domain)            │
│  - Pure Rust types                      │
│  - No external dependencies             │
│  - Traits (Ports)                       │
│  - CanonicalMessage                     │
└──────────────┬──────────────────────────┘
               │ Implements
┌──────────────▼──────────────────────────┐
│        src/adapters/ (Infrastructure)   │
│  - OpenAI client                        │
│  - Qdrant client                        │
│  - Sled store                           │
│  - External type mapping                │
└─────────────────────────────────────────┘
```

**Key Rules**:
- `src/core/` MUST NOT import `axum`, `reqwest`, `qdrant`, or `openai`
- All external types are mapped to `CanonicalMessage` at adapter boundaries
- Traits defined in core, implementations in adapters

### 2. Canonical Message Model

All domain communication uses `CanonicalMessage` - a pure Rust type with no external dependencies:

```rust
// In src/core/types.rs
pub struct CanonicalMessage {
    pub id: MessageId,
    pub role: Role,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    // ... no async_openai types here
}
```

**Mapping Strategy**:
- API receives JSON → DTO → `CanonicalMessage`
- `CanonicalMessage` → OpenAI types (in adapter)
- OpenAI response → `CanonicalMessage` (in adapter)

### 3. Actor Model with Message Passing

Communication uses `tokio::sync::mpsc` channels, avoiding shared mutable state:

```rust
// Actor pattern
struct Actor {
    rx: mpsc::Receiver<CanonicalMessage>,
    state: AgentState,
}

// No Arc<Mutex<T>> - use channels instead
```

**Benefits**:
- No data races (Rust compiler guarantees)
- Clear ownership boundaries
- Cancellation safety with `tokio::select!`

### 4. State Machine Orchestration

The orchestrator maintains explicit state transitions:

```
Idle → Thinking → ToolCall → Reflecting → Idle
  ↑                                         │
  └─────────────────────────────────────────┘
```

States are explicit enum variants, not implicit through message flow.

## Module Structure

### src/core/ - Domain Layer

**Purpose**: Pure business logic, no infrastructure dependencies

**Files**:
- `types.rs`: Domain types (`CanonicalMessage`, `AgentState`, `Role`)
- `traits.rs`: Port definitions (`LLMProvider`, `VectorStore`)
- `error.rs`: Domain errors using `thiserror`
- `mod.rs`: Module exports

**Rules**:
- Only dependencies: `serde`, `uuid`, `thiserror`, `chrono`
- NO async runtime imports
- NO HTTP client imports
- NO database imports

### src/adapters/ - Infrastructure Layer

**Purpose**: Implementations of core traits, external service integration

**Files**:
- `openai.rs`: `OpenAIClient` implementing `LLMProvider`
- `qdrant.rs`: `QdrantRepo` implementing `VectorStore`
- `sled.rs`: `SledStore` for medium-term memory
- `mod.rs`: Module exports

**Responsibilities**:
- Map external types to `CanonicalMessage`
- Handle network/IO errors
- Convert `SentinelError` from external errors

### src/engine/ - Orchestration Layer

**Purpose**: Actor loops, state management, coordination

**Files**:
- `actor.rs`: Main actor loop with `tokio::select!`
- `supervisor.rs`: Process manager, zombie detection
- `mod.rs`: Module exports

**Pattern**:
```rust
async fn actor_loop(rx: mpsc::Receiver<Message>) {
    let mut state = AgentState::Idle;
    loop {
        tokio::select! {
            msg = rx.recv() => {
                // Handle message, update state
                state = process_message(msg, state).await;
            }
            // ... other branches
        }
    }
}
```

### src/api/ - Gateway Layer

**Purpose**: HTTP server, request/response handling

**Files**:
- `routes.rs`: Axum route definitions
- `middleware.rs`: Request middleware (auth, logging)
- `mod.rs`: Module exports

**Flow**:
1. HTTP request → DTO
2. Validate DTO
3. Convert to `CanonicalMessage`
4. Send to engine via channel
5. Receive response
6. Stream back to client

### src/memory/ - Memory Management

**Purpose**: Three-tier memory consolidation

**Files**:
- `manager.rs`: `MemoryManager` coordinating tiers
- `mod.rs`: Module exports

**Three Tiers**:
1. **Short-Term**: In-memory conversation history
2. **Medium-Term**: Sled database (summarized conversations)
3. **Long-Term**: Qdrant vector store (embeddings)

**Consolidation Trigger**:
- When token count exceeds threshold
- Background task (Dreamer) summarizes
- Short-term → Medium-term (Sled)
- Medium-term summaries → Long-term (Qdrant embeddings)

### src/telemetry/ - Observability

**Purpose**: Logging, tracing, metrics

**Files**:
- `mod.rs`: Telemetry setup

**Implementation**:
- Use `tracing` crate
- OpenTelemetry integration
- Structured logging

## Agent Roles

### 1. The Sentinel (Orchestrator)

**Location**: `src/engine/actor.rs`

**Responsibilities**:
- Manages main event loop
- Handles state transitions
- Coordinates memory access
- Executes LLM provider calls

**Pattern**: Actor with `mpsc::Receiver<CanonicalMessage>`

### 2. The Supervisor

**Location**: `src/engine/supervisor.rs`

**Responsibilities**:
- Spawns new agents
- Monitors agent health
- Detects "zombie" processes (stuck > 60s)
- Handles restarts

**Pattern**: Supervisor actor monitoring child actors

### 3. The Dreamer (Memory Manager)

**Location**: `src/memory/manager.rs`

**Responsibilities**:
- Background task monitoring token counts
- Triggers memory consolidation
- Summarizes Short-Term → Medium-Term
- Embeddings Medium-Term → Long-Term

**Pattern**: Background task with periodic checks

## Data Flow

### Message Flow

```
Client Request (JSON)
    ↓
API Gateway (routes.rs)
    ↓
DTO Validation
    ↓
CanonicalMessage (core/types.rs)
    ↓
Channel → Engine (engine/actor.rs)
    ↓
State Machine Processing
    ↓
LLM Provider (adapters/openai.rs)
    ↓
CanonicalMessage Response
    ↓
Channel → API Gateway
    ↓
HTTP Response (Streaming)
```

### Memory Consolidation Flow

```
Short-Term Memory (In-Memory)
    ↓ (Token count > threshold)
Dreamer Task Triggered
    ↓
Summarize via LLM
    ↓
Medium-Term Memory (Sled)
    ↓ (Periodic consolidation)
Generate Embeddings
    ↓
Long-Term Memory (Qdrant)
```

## Error Handling Strategy

### Core Layer (`src/core/`)
- Use `thiserror` for domain errors
- Define `SentinelError` enum
- No context, just error types

```rust
#[derive(thiserror::Error, Debug)]
pub enum SentinelError {
    #[error("Invalid state transition")]
    InvalidStateTransition,
    // ...
}
```

### Application Layer (`src/api/`, `src/engine/`)
- Use `anyhow` for error context
- Convert `SentinelError` to HTTP responses
- Add context with `.context("Failed to...")?`

```rust
fn handle_error(error: anyhow::Error) -> axum::response::Response {
    // Convert to HTTP status code
    // Log with context
}
```

## Concurrency Patterns

### Actor Communication

All actors communicate via channels:

```rust
let (tx, rx) = mpsc::channel(32);
let actor = tokio::spawn(actor_loop(rx));
tx.send(message).await?;
```

### Cancellation Safety

Use `tokio::select!` for cancellation:

```rust
tokio::select! {
    msg = rx.recv() => { /* handle */ }
    _ = shutdown_signal => { break; }
    timeout = tokio::time::sleep(Duration::from_secs(60)) => {
        // Handle timeout
    }
}
```

### No Shared Mutable State

Avoid `Arc<Mutex<T>>`. Instead:
- Use channels for communication
- Pass ownership when possible
- Use `Cow<T>` for borrowed/cloned data

## Configuration Management

### Environment Variables
- API keys from `ENV` only
- Use `secrecy::Secret<String>` for sensitive data
- Load via `config` crate

### Configuration File
- `config.toml` for non-sensitive settings
- Merged with environment variables
- Type-safe configuration structs

## Testing Strategy

### Unit Tests
- Co-locate with code in `mod tests`
- Mock traits using `mockall`
- Test domain logic without infrastructure

### Integration Tests
- In `tests/` directory
- Real Qdrant instance (Docker/Testcontainers)
- Real Sled database
- Don't mock storage in integration tests

## Design Decisions

### Why Hexagonal Architecture?
- **Testability**: Mock adapters easily
- **Flexibility**: Swap OpenAI for Anthropic without changing core
- **Clarity**: Clear boundaries reduce coupling

### Why Canonical Message Model?
- **Independence**: Core doesn't depend on external APIs
- **Stability**: External API changes don't break domain
- **Testability**: Test with simple Rust types

### Why Actor Model?
- **Safety**: Rust guarantees no data races
- **Scalability**: Easy to distribute across threads
- **Clarity**: Explicit message flow

### Why Three-Tier Memory?
- **Efficiency**: Recent context fast (in-memory)
- **Persistence**: Medium-term survives restarts (Sled)
- **Searchability**: Long-term enables semantic search (Qdrant)

## Research-Informed Design Decisions

This architecture incorporates findings from research into existing Rust orchestrator frameworks. Key decisions based on that research:

### Patterns Adopted from Existing Frameworks

1. **Channel-Based Communication** (from ccswarm)
   - All actor communication uses `tokio::sync::mpsc`
   - No shared mutable state (`Arc<Mutex>`)
   - Matches industry best practices for Rust async actors

2. **Agent Factory Pattern** (from Swarm)
   - Dynamic agent creation for Supervisor role
   - Runtime instantiation patterns
   - Lifecycle management strategies

3. **Performance Optimization** (from Swarms-rs)
   - Zero-cost abstractions throughout
   - Efficient async task spawning
   - Resource pooling patterns

4. **Type Safety** (from Orka)
   - Type-state patterns where applicable
   - NewType pattern for IDs
   - Compile-time guarantees

### Unique Sentinel Requirements

Unlike existing frameworks, Sentinel implements:

1. **Strict Hexagonal Architecture**: More rigorous than most frameworks
2. **Three-Tier Memory System**: Unique consolidation strategy
3. **Canonical Message Model**: Stricter domain boundaries
4. **Explicit State Machine**: More rigid than workflow-based approaches
5. **Cursor 2.0 Integration**: Specific platform requirements

See [Research Comparison](./research/rust_orchestrators_comparison.md) for detailed analysis.

## References

- [Research: Rust Orchestrators Comparison](./research/rust_orchestrators_comparison.md)
- [Build Plan: Cursor Recursive Plan](./cursor_plan.md)
- [Agent Roles: AGENTS.md](../AGENTS.md)
- [Cursor Rules: .cursor/rules/](../.cursor/rules/)
- [Main Branch Sync Process](./MAIN_BRANCH_SYNC.md)

