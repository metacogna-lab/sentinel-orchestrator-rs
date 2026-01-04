# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Sentinel Orchestrator** is a production-grade agentic system built with military-grade idiomatic Rust, following strict Hexagonal Architecture principles. The system orchestrates AI agents with resilience patterns including backpressure handling, circuit breakers, and budget kill switches.

## Core Architecture Principles

### Hexagonal Architecture (Ports and Adapters)

The codebase enforces strict separation of concerns:

- **`core/`** - Pure domain logic with **ZERO external I/O dependencies**
  - `types.rs` - Domain types (CanonicalMessage, AgentState, etc.)
  - `traits.rs` - Port interfaces (LLMProvider, VectorStore, etc.)
  - `error.rs` - Domain errors using `thiserror`
  - **CRITICAL**: No `tokio`, `reqwest`, `axum`, or any infrastructure crates allowed in core

- **`adapters/`** - Infrastructure implementations
  - `openai.rs` - OpenAI API adapter implementing LLM traits
  - `qdrant.rs` - Qdrant vector database adapter
  - `sled.rs` - Sled embedded KV store adapter
  - Adapters implement traits defined in `core/traits.rs`

- **`engine/`** - Agent orchestration runtime
  - `actor.rs` - Agent actor model with event loops
  - `supervisor.rs` - Supervisor for agent lifecycle management
  - Uses `tokio::mpsc` channels for backpressure control

- **`memory/`** - Memory hierarchy management
  - `manager.rs` - Short/Medium/Long-term memory coordination

- **`api/`** - HTTP gateway layer
  - `routes.rs` - Axum route handlers
  - `middleware.rs` - Tower middleware stack

- **`telemetry/`** - Observability infrastructure
  - Structured logging with `tracing`
  - OpenTelemetry integration for distributed tracing

## Military-Grade Rust Standards

### Error Handling (Non-Negotiable)

1. **Use `thiserror` for domain errors** in `src/core`:
   ```rust
   use thiserror::Error;

   #[derive(Error, Debug)]
   pub enum CoreError {
       #[error("Agent state transition failed: {0}")]
       InvalidTransition(String),
   }
   ```

2. **Use `anyhow` for application errors** in `main.rs` and binaries:
   ```rust
   use anyhow::{Context, Result};

   fn main() -> Result<()> {
       load_config().context("Failed to load configuration")?;
       Ok(())
   }
   ```

3. **FORBIDDEN**:
   - `unwrap()` or `expect()` in production code paths
   - Silently swallowing errors (always propagate with `?`)
   - Panicking in library code

4. **Always use `Result<T, E>`** for fallible operations - make failure explicit in types

### Async Patterns

1. **Prefer `async fn` over manual `Future` implementations**
   ```rust
   async fn process_agent_message(&self, msg: Message) -> Result<Response> { }
   ```

2. **Use bounded channels for backpressure**:
   ```rust
   let (tx, rx) = tokio::sync::mpsc::channel::<AgentEvent>(100);
   ```
   - Never use unbounded channels in production
   - Prevents OOM under high load

3. **Concurrent execution patterns**:
   - Sequential when dependencies exist: `let x = foo().await; bar(x).await;`
   - Parallel when independent: `tokio::try_join!(foo(), bar())`

4. **Cancellation and timeouts**:
   ```rust
   tokio::select! {
       result = agent.process() => handle_result(result),
       _ = timeout(Duration::from_secs(30)) => handle_timeout(),
   }
   ```

5. **Use `tokio::spawn` for concurrent tasks** - keep runtime async, never block threads

### Memory Management

1. **Shared ownership**: `Arc<T>` for thread-safe reference counting (never `Rc<T>`)
2. **String parameters**: Prefer `&str` over `String` to avoid unnecessary allocations
3. **Pre-allocate vectors**: Use `Vec::with_capacity(n)` when size is known
4. **Minimize clones**: Use references or `Arc` instead of `.clone()` in hot paths
5. **Inline hints**: Use `#[inline]` sparingly, only for proven hot paths

### Testing

1. **Unit tests with `mockall`** for trait mocking:
   ```rust
   use mockall::automock;

   #[automock]
   #[async_trait]
   pub trait LLMProvider {
       async fn complete(&self, prompt: &str) -> Result<String>;
   }
   ```

2. **Async tests** require `#[tokio::test]`:
   ```rust
   #[tokio::test]
   async fn test_agent_lifecycle() {
       let supervisor = Supervisor::new();
       let agent = supervisor.spawn_agent().await.unwrap();
       assert!(agent.is_running());
   }
   ```

3. **Integration tests** in `tests/` directory - use Docker for external dependencies
4. **Test error paths**, not just happy paths - verify error handling works

## Development Commands

### Building

```bash
# Build in development mode
cargo build

# Build with optimizations (release mode)
cargo build --release

# Build specific crate (if workspace grows)
cargo build -p sentinel
```

### Running

```bash
# Run the binary
cargo run

# Run with release optimizations
cargo run --release

# Run with environment variables
RUST_LOG=debug cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_agent_lifecycle

# Run tests with logging
RUST_LOG=debug cargo test

# Run tests in specific module
cargo test engine::actor
```

### Linting and Code Quality

```bash
# Run Clippy (required before commits)
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check

# Auto-format code
cargo fmt

# Check for unused dependencies
cargo udeps  # Requires: cargo install cargo-udeps

# Security audit
cargo audit  # Requires: cargo install cargo-audit
```

### Documentation

```bash
# Generate and open documentation
cargo doc --open

# Generate docs for all dependencies
cargo doc --open --no-deps
```

## Critical Patterns for Agentic Systems

### 1. Interrupt Persistence
Agents must be able to save state and resume after interruption:
```rust
pub struct AgentCheckpoint {
    pub agent_id: Uuid,
    pub state: AgentState,
    pub message_history: Vec<CanonicalMessage>,
}
```

### 2. Zombie Detection
Supervisor must detect and clean up dead agents:
```rust
// Use timeouts with select! to detect unresponsive agents
tokio::select! {
    _ = agent_heartbeat.recv() => { /* agent alive */ },
    _ = tokio::time::sleep(HEARTBEAT_TIMEOUT) => { /* zombie detected */ }
}
```

### 3. Budget Kill Switches
Prevent runaway costs with circuit breakers:
```rust
pub struct BudgetGuard {
    max_tokens: usize,
    consumed: AtomicUsize,
}

impl BudgetGuard {
    pub fn check_and_increment(&self, tokens: usize) -> Result<()> {
        let current = self.consumed.fetch_add(tokens, Ordering::SeqCst);
        if current + tokens > self.max_tokens {
            Err(CoreError::BudgetExceeded)
        } else {
            Ok(())
        }
    }
}
```

### 4. Deterministic Replay
All agent interactions must be reproducible:
- Use strict JSON schemas for all messages
- Validate input/output with `serde_json` schemas
- Store all events with timestamps in `tracing` spans

### 5. Backpressure Handling
Bounded channels throughout the system:
```rust
// Engine event loop
let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<EngineEvent>(100);

// This will block sender if queue is full - prevents OOM
event_tx.send(event).await?;
```

## Configuration

### Environment Variables

Create a `.env` file (never commit this):
```bash
OPENAI_API_KEY=sk-...
QDRANT_URL=http://localhost:6334
SLED_PATH=./data/sled
RUST_LOG=info,sentinel=debug
```

Load with `dotenvy` in `main.rs`:
```rust
dotenvy::dotenv().ok();
```

### Secrets Management

Use `secrecy` crate for sensitive data:
```rust
use secrecy::{Secret, ExposeSecret};

pub struct Config {
    pub api_key: Secret<String>,
}

// Access only when needed
let key = config.api_key.expose_secret();
```

## Observability

### Structured Logging

All operations wrapped in `tracing` spans:
```rust
use tracing::{info_span, instrument};

#[instrument(skip(self))]
pub async fn process_message(&self, msg: Message) -> Result<Response> {
    let span = info_span!("process_message", agent_id = %self.id);
    let _guard = span.enter();

    info!("Processing message");
    // ... implementation
}
```

### Log Levels

- `error` - Unrecoverable errors requiring intervention
- `warn` - Recoverable errors, circuit breaker trips
- `info` - Agent lifecycle events, message processing
- `debug` - Detailed execution flow
- `trace` - Full message contents (avoid in production)

## Dependency Philosophy

### Core Dependencies (Locked)

- **Async Runtime**: `tokio` (version 1.x)
- **HTTP Server**: `axum` (version 0.7)
- **Serialization**: `serde` + `serde_json`
- **Error Handling**: `thiserror` (core), `anyhow` (app)
- **Observability**: `tracing` ecosystem
- **AI Providers**: `async-openai`, `qdrant-client`
- **Storage**: `sled` for embedded KV

### Adding New Dependencies

1. Justify the need - avoid "nice to have" crates
2. Check maintenance status and security advisories
3. Prefer crates with `#![forbid(unsafe_code)]` when possible
4. Update `Cargo.toml` with version constraints: `crate = "1.2"` (allows 1.2.x)

## Performance Targets

- **Agent Response Latency**: p99 < 500ms (excluding LLM API calls)
- **Throughput**: 1000+ messages/second per supervisor
- **Memory**: Bounded by channel sizes, no unbounded growth
- **CPU**: < 5% overhead for orchestration logic (excluding LLM inference)

## Common Pitfalls

1. **Mixing I/O in core** - Keep `core/` pure, put I/O in `adapters/`
2. **Unbounded channels** - Always use `mpsc::channel(N)` with explicit capacity
3. **Blocking in async** - Use `tokio::task::spawn_blocking` for CPU-bound work
4. **Clone-heavy code** - Profile before optimizing, but prefer `Arc` for shared data
5. **Missing error context** - Always use `.context()` when propagating errors
6. **Implicit panics** - Avoid `[index]` access, use `.get(index)` instead

## IDE Setup

### VS Code (Recommended)

Install extensions:
- `rust-analyzer` - Language server
- `crates` - Cargo.toml dependency management
- `Even Better TOML` - TOML syntax

Settings (`.vscode/settings.json`):
```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.cargo.features": "all"
}
```

### Cursor

Uses `.cursor/rules/*.mdc` for project-specific guidelines:
- `error-handling.mdc` - Error handling patterns
- `architecture.mdc` - Hexagonal architecture enforcement
- `async-patterns.mdc` - Async/await best practices
- `testing.mdc` - Test patterns with mockall
- `performance.mdc` - Performance optimization rules
