# Sentinel Orchestrator - Product Requirements Document

## Overview

Sentinel Orchestrator is a production-grade agentic system built with military-grade idiomatic Rust, following strict Hexagonal Architecture principles. The system orchestrates AI agents with resilience patterns including backpressure handling, circuit breakers, and budget kill switches.

## Core Requirements

### 1. Hexagonal Architecture (Ports & Adapters)

**Requirement**: Strict separation between domain logic and infrastructure.

- **`src/core/`** - Pure domain logic with ZERO external I/O dependencies
  - `types.rs` - Domain types (CanonicalMessage, AgentState, etc.)
  - `traits.rs` - Port interfaces (LLMProvider, VectorStore, etc.)
  - `error.rs` - Domain errors using `thiserror`
  - **CRITICAL**: No `tokio`, `reqwest`, `axum`, or any infrastructure crates allowed in core

- **`src/adapters/`** - Infrastructure implementations
  - `openai.rs` - OpenAI API adapter implementing LLM traits
  - `qdrant.rs` - Qdrant vector database adapter
  - `sled.rs` - Sled embedded KV store adapter
  - Adapters implement traits defined in `core/traits.rs`

### 2. Canonical Message Model

**Requirement**: All domain communication uses `CanonicalMessage` - a pure Rust type with no external dependencies.

- Map external types to `CanonicalMessage` at adapter boundaries
- API receives JSON → DTO → `CanonicalMessage`
- `CanonicalMessage` → OpenAI types (in adapter)
- OpenAI response → `CanonicalMessage` (in adapter)

### 3. Actor Model with Message Passing

**Requirement**: Communication uses `tokio::sync::mpsc` channels, avoiding shared mutable state.

- Use bounded channels (`mpsc::channel(N)`) for backpressure control
- No `Arc<Mutex<T>>` - use channels instead
- Clear ownership boundaries
- Cancellation safety with `tokio::select!`

### 4. State Machine Orchestration

**Requirement**: Explicit state transitions.

```
Idle → Thinking → ToolCall → Reflecting → Idle
  ↑                                         │
  └─────────────────────────────────────────┘
```

- States are explicit enum variants, not implicit through message flow
- State transitions managed by orchestrator

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

## Memory System

### Three-Tier Memory Consolidation

**Requirement**: Short-Term → Medium-Term → Long-Term memory hierarchy.

1. **Short-Term**: In-memory conversation history
2. **Medium-Term**: Sled database (summarized conversations)
3. **Long-Term**: Qdrant vector store (embeddings)

**Consolidation Trigger**:
- When token count exceeds threshold
- Background task (Dreamer) summarizes
- Short-term → Medium-term (Sled)
- Medium-term summaries → Long-term (Qdrant embeddings)

## Error Handling

### Core Layer (`src/core/`)
- Use `thiserror` for domain errors
- Define `SentinelError` enum
- No context, just error types

### Application Layer (`src/api/`, `src/engine/`)
- Use `anyhow` for error context
- Convert `SentinelError` to HTTP responses
- Add context with `.context("Failed to...")?`

**FORBIDDEN**:
- `unwrap()` or `expect()` in production code paths
- Silently swallowing errors (always propagate with `?`)
- Panicking in library code

## Resilience Patterns

### 1. Backpressure Handling
- Bounded channels throughout the system
- Never use unbounded channels in production
- Prevents OOM under high load

### 2. Zombie Detection
- Supervisor must detect and clean up dead agents
- Use timeouts with `select!` to detect unresponsive agents
- Timeout threshold: > 60 seconds

### 3. Budget Kill Switches
- Prevent runaway costs with circuit breakers
- Track token consumption
- Fail fast when budget exceeded

### 4. Interrupt Persistence
- Agents must be able to save state and resume after interruption
- Checkpoint mechanism for agent state

## Technical Requirements

### Async Patterns
- Prefer `async fn` over manual `Future` implementations
- Use `tokio::select!` for cancellation and timeouts
- Use `tokio::spawn` for concurrent tasks
- Never block threads in async code

### Testing
- Unit tests with `mockall` for trait mocking
- Async tests require `#[tokio::test]`
- Integration tests in `tests/` directory
- Test error paths, not just happy paths

### Observability
- Structured logging with `tracing`
- OpenTelemetry integration for distributed tracing
- All operations wrapped in `tracing` spans

## Performance Targets

- **Agent Response Latency**: p99 < 500ms (excluding LLM API calls)
- **Throughput**: 1000+ messages/second per supervisor
- **Memory**: Bounded by channel sizes, no unbounded growth
- **CPU**: < 5% overhead for orchestration logic (excluding LLM inference)

## Integration Requirements

### LLM Provider Trait
- Trait-based abstraction in core
- Implementations in adapters
- Support for OpenAI (initial implementation)

### Vector Store Trait
- Trait-based abstraction for Qdrant
- Support semantic search via embeddings

### Configuration
- Environment variables for API keys
- Use `secrecy::Secret<String>` for sensitive data
- `config.toml` for non-sensitive settings

## Development Phases

### Phase 1: Core Domain
- Implement strict hexagonal boundaries
- Create canonical message model (no external deps)
- Define core traits and error types

### Phase 2: Actor System
- Implement channel-based communication
- Implement explicit state machine
- Create actor event loops

### Phase 3: Memory System
- Design three-tier memory system
- Implement consolidation logic
- Token counting for triggers

### Phase 4: Integration
- Standard trait patterns for LLM/VectorStore
- Cursor 2.0 integration
- Maintain strict adapter boundaries

## References

- [Architecture Documentation](../docs/architecture.md)
- [Research Comparison](../docs/research/rust_orchestrators_comparison.md)
- [CLAUDE.md](../CLAUDE.md)

