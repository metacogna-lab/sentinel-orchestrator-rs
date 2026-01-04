# Product Requirements Document (PRD)
## Sentinel Rust Orchestrator
**Classification**: Internal Development  
**Version**: 1.0  
**Date**: 2025-01-20  
**Author**: Sentinel Development Team

---

## 1. EXECUTIVE SUMMARY

### 1.1 Purpose
This document defines the requirements, specifications, and implementation phases for the Sentinel Rust Orchestrator, a production-grade multi-agent orchestration system designed for high-performance AI agent coordination with strict architectural boundaries.

### 1.2 Scope
The Sentinel Orchestrator provides:
- Multi-agent coordination through message-passing actors
- Three-tier memory consolidation system (Short/Medium/Long-term)
- Strict Hexagonal Architecture (Ports & Adapters)
- Integration with LLM providers via trait-based abstraction
- Production-ready observability and resilience features

### 1.3 Success Criteria
- System processes 1000+ concurrent agent interactions per second
- Memory consolidation reduces context size by 80% without information loss
- Zero data races (compiler-guaranteed via Rust ownership)
- 99.9% uptime under normal load conditions
- All phases meet acceptance criteria before progression

---

## 2. MISSION STATEMENT

### 2.1 Primary Objective
Build a high-performance, production-ready Rust orchestrator that coordinates AI agents through strict hexagonal architecture, ensuring type safety, memory safety, and operational excellence.

### 2.2 Core Principles
1. **Architectural Purity**: Strict separation between domain (`src/core/`) and infrastructure (`src/adapters/`)
2. **Type Safety**: Compile-time guarantees via Rust type system
3. **Memory Safety**: Zero-cost abstractions with no runtime overhead
4. **Observability**: Complete visibility into system behavior
5. **Resilience**: Graceful degradation under load

---

## 3. SYSTEM OVERVIEW

### 3.1 Architecture Pattern
**Hexagonal Architecture (Ports & Adapters)**
- **Core Domain** (`src/core/`): Pure Rust types, no external I/O dependencies
- **Adapters** (`src/adapters/`): Infrastructure implementations (OpenAI, Qdrant, Sled)
- **Engine** (`src/engine/`): Actor loops, state machines, orchestration
- **API** (`src/api/`): HTTP gateway (Axum)
- **Memory** (`src/memory/`): Three-tier consolidation system
- **Telemetry** (`src/telemetry/`): Observability layer

### 3.2 Agent Roles
1. **Sentinel (Orchestrator)**: Central coordination actor managing event loops and state transitions
2. **Supervisor**: Process manager detecting zombie processes (>60s timeout)
3. **Dreamer (Memory Manager)**: Background task for memory consolidation

### 3.3 Data Flow
```
Client Request (JSON)
    ↓
API Gateway (Axum)
    ↓
CanonicalMessage (Domain)
    ↓
Channel (tokio::sync::mpsc)
    ↓
Engine Actor Loop
    ↓
State Machine Processing
    ↓
LLM Provider (Adapter)
    ↓
Response Channel
    ↓
HTTP Stream Response
```

---

## 4. ARCHITECTURE CONSTRAINTS

### 4.1 Core Domain Rules
- `src/core/` MUST NOT import: `axum`, `reqwest`, `qdrant`, `openai`, `tokio`
- Only allowed dependencies: `serde`, `uuid`, `thiserror`, `chrono`
- All external types mapped to `CanonicalMessage` at adapter boundaries
- Traits defined in core, implementations in adapters

### 4.2 Concurrency Rules
- Use `tokio::sync::mpsc` channels for all actor communication
- Prohibited: `Arc<Mutex<T>>` patterns (use message passing instead)
- Use `tokio::select!` for cancellation safety
- Bounded channels required for backpressure

### 4.3 Error Handling Rules
- `src/core/`: Use `thiserror` for domain errors
- `src/api/`, `src/engine/`: Use `anyhow` for application errors
- No `unwrap()` or `expect()` in production code paths

### 4.4 Testing Rules
- Unit tests: Co-locate with code, use `mockall` for trait mocking
- Integration tests: Real infrastructure (Docker/Testcontainers)
- Test error paths, not just happy paths

---

## 5. TECHNICAL STACK

### 5.1 Approved Dependencies (Cargo.toml Only)

#### 5.1.1 Async Runtime & HTTP
- `tokio = "1"` (features: ["full"])
- `axum = "0.7"` (features: ["macros"])
- `tower = "0.4"` (features: ["timeout", "limit"])
- `tower-http = "0.5"` (features: ["trace", "cors"])

#### 5.1.2 Data & Serialization
- `serde = "1"` (features: ["derive"])
- `serde_json = "1"`
- `uuid = "1"` (features: ["v4", "serde"])
- `chrono = "0.4"` (features: ["serde"])

#### 5.1.3 AI & Infrastructure Drivers
- `async-openai = "0.23"`
- `qdrant-client = "1.10"`
- `sled = "0.34"`
- `reqwest = "0.12"` (features: ["json", "stream"])

#### 5.1.4 Error Handling & Configuration
- `thiserror = "1"` (core domain errors)
- `anyhow = "1"` (application errors)
- `config = "0.14"` (configuration management)
- `secrecy = "0.8"` (features: ["serde"]) (API key protection)

#### 5.1.5 Observability
- `tracing = "0.1"`
- `tracing-subscriber = "0.3"` (features: ["env-filter", "json"])
- `opentelemetry = "0.22"`

#### 5.1.6 Utilities
- `async-trait = "0.1"`
- `dotenvy = "0.15"`

#### 5.1.7 Development Dependencies
- `mockall = "0.13"` (trait mocking)
- `tokio-test = "0.4"` (async test utilities)

### 5.2 Prohibited Dependencies
- Any crate not explicitly listed in Section 5.1
- Blocking I/O crates (use Tokio async equivalents)
- Runtime reflection or dynamic dispatch (unless via traits)

---

## 6. MVP PHASES (1-3)

## 6.1 PHASE 1: CORE DOMAIN FOUNDATION

### 6.1.1 Objectives
Establish the pure domain layer with canonical message model, state machine definitions, and trait interfaces. No infrastructure dependencies.

### 6.1.2 Deliverables

#### 6.1.2.1 Domain Types (`src/core/types.rs`)
- `CanonicalMessage` struct:
  - `id: MessageId` (NewType wrapper around `Uuid`)
  - `role: Role` enum (`User`, `Assistant`, `System`)
  - `content: String`
  - `timestamp: DateTime<Utc>`
  - `metadata: HashMap<String, String>` (optional)
- `AgentState` enum:
  - `Idle`
  - `Thinking`
  - `ToolCall`
  - `Reflecting`
- `Role` enum: `User`, `Assistant`, `System`
- `MessageId` NewType: `pub struct MessageId(pub Uuid);`

#### 6.1.2.2 Error Definitions (`src/core/error.rs`)
- `SentinelError` enum using `thiserror`:
  - `InvalidStateTransition { from: AgentState, to: AgentState }`
  - `InvalidMessage { reason: String }`
  - `DomainViolation { rule: String }`
- All variants implement `Error` trait via `thiserror`

#### 6.1.2.3 Core Traits (`src/core/traits.rs`)
- `LLMProvider` trait (using `async-trait`):
  - `async fn complete(&self, messages: Vec<CanonicalMessage>) -> Result<CanonicalMessage, SentinelError>`
  - `async fn stream(&self, messages: Vec<CanonicalMessage>) -> Result<impl Stream<Item = Result<String, SentinelError>>, SentinelError>`
- `VectorStore` trait (using `async-trait`):
  - `async fn upsert(&self, id: MessageId, embedding: Vec<f32>, metadata: HashMap<String, String>) -> Result<(), SentinelError>`
  - `async fn search(&self, query_embedding: Vec<f32>, limit: usize) -> Result<Vec<MessageId>, SentinelError>`
- All traits MUST be mockable with `mockall`

#### 6.1.2.4 Module Structure (`src/core/mod.rs`)
- Export all public types and traits
- No re-exports of infrastructure crates

### 6.1.3 Acceptance Criteria
1. `cargo build` succeeds with zero errors
2. `cargo test` executes with 100% pass rate
3. `cargo clippy -- -D warnings` passes with zero warnings
4. All types implement `Serialize`/`Deserialize` where required
5. Unit tests exist for:
   - `CanonicalMessage` creation and validation
   - State transition logic
   - Error variant instantiation
6. Zero infrastructure dependencies in `src/core/` verified by `cargo tree`

### 6.1.4 Dependencies
- None (Phase 1 is the foundation)

### 6.1.5 Risks
- **Risk**: Over-engineering domain types
- **Mitigation**: Keep types minimal, extend only when needed
- **Risk**: Trait design too restrictive
- **Mitigation**: Review against adapter requirements before finalization

---

## 6.2 PHASE 2: ACTOR SYSTEM & BASIC ORCHESTRATION

### 6.2.1 Objectives
Implement the actor model with state machine orchestration, channel-based communication, and basic supervisor functionality for zombie detection.

### 6.2.2 Deliverables

#### 6.2.2.1 Actor Loop (`src/engine/actor.rs`)
- `Actor` struct:
  - `rx: mpsc::Receiver<CanonicalMessage>` (bounded channel, size: 32)
  - `state: AgentState`
  - `llm: Arc<dyn LLMProvider>` (trait object)
  - `tx: mpsc::Sender<CanonicalMessage>` (response channel)
- `actor_loop(rx, llm, tx) -> Result<(), SentinelError>`:
  - Uses `tokio::select!` for cancellation safety
  - Handles state transitions: `Idle -> Thinking -> ToolCall -> Reflecting -> Idle`
  - Processes messages via `LLMProvider::complete()`
  - Sends responses via channel
  - Timeout handling: 60 second max per message

#### 6.2.2.2 State Machine (`src/engine/actor.rs`)
- `transition_state(current: AgentState, event: MessageEvent) -> Result<AgentState, SentinelError>`
- Valid transitions:
  - `Idle` + `MessageReceived` → `Thinking`
  - `Thinking` + `LLMResponse` → `ToolCall` or `Reflecting`
  - `ToolCall` + `ToolResult` → `Reflecting`
  - `Reflecting` + `Complete` → `Idle`
- Invalid transitions return `SentinelError::InvalidStateTransition`

#### 6.2.2.3 Supervisor (`src/engine/supervisor.rs`)
- `Supervisor` struct:
  - Monitors active actors via `HashMap<ActorId, ActorHandle>`
  - Spawns new actors on demand
  - Health check interval: 10 seconds
- `supervise_loop()` function:
  - Detects zombie processes (no response >60s)
  - Terminates and restarts zombie actors
  - Logs all supervisor actions via `tracing`
- Zombie detection:
  - Track last activity timestamp per actor
  - If `now - last_activity > 60s`, mark as zombie
  - Send termination signal, spawn replacement

#### 6.2.2.4 Engine Module (`src/engine/mod.rs`)
- Export `Actor`, `Supervisor`, `actor_loop`, `supervise_loop`
- Public API: `spawn_actor(llm: Arc<dyn LLMProvider>) -> (ActorHandle, mpsc::Sender<CanonicalMessage>)`

### 6.2.3 Acceptance Criteria
1. Single agent processes message end-to-end:
   - Send message via channel
   - Actor transitions states correctly
   - Response received via channel
2. State machine enforces valid transitions only
3. Supervisor detects and restarts zombie actor within 70 seconds
4. `tokio::select!` handles cancellation correctly (test with timeout)
5. All channels are bounded (prevent OOM)
6. Integration test: Spawn 10 actors, verify all process messages concurrently

### 6.2.4 Dependencies
- Phase 1 complete (Core Domain)
- Mock `LLMProvider` implementation for testing

### 6.2.5 Risks
- **Risk**: Deadlock in actor communication
- **Mitigation**: Bounded channels, timeout handling, extensive testing
- **Risk**: State machine complexity
- **Mitigation**: Explicit state enum, transition validation

---

## 6.3 PHASE 3: API GATEWAY & ADAPTER INTEGRATION

### 6.3.1 Objectives
Implement HTTP API gateway with OpenAI adapter, enabling external clients to send requests and receive streaming responses.

### 6.3.2 Deliverables

#### 6.3.2.1 API Routes (`src/api/routes.rs`)
- `POST /v1/chat/completions` endpoint:
  - Accepts JSON request body (OpenAI-compatible format)
  - Validates request structure
  - Converts to `Vec<CanonicalMessage>`
  - Sends to engine via channel
  - Streams response back (Server-Sent Events or chunked transfer)
- `GET /health` endpoint:
  - Returns `200 OK` with JSON: `{"status": "healthy"}`
- Request DTO:
  - `ChatCompletionRequest` struct (deserializable from JSON)
  - Field validation (required fields, type checks)

#### 6.3.2.2 Middleware (`src/api/middleware.rs`)
- Request logging middleware:
  - Log incoming requests via `tracing`
  - Include: method, path, request_id (UUID)
- Error handling middleware:
  - Catch panics (convert to 500)
  - Convert `SentinelError` to HTTP status codes
  - Convert `anyhow::Error` to 500 with error message
- CORS middleware (via `tower-http`):
  - Allow all origins in development
  - Configurable in production

#### 6.3.2.3 OpenAI Adapter (`src/adapters/openai.rs`)
- `OpenAIClient` struct:
  - `client: async_openai::Client`
  - `model: String` (configurable, default: "gpt-4")
- Implements `LLMProvider` trait:
  - `complete()`: Maps `CanonicalMessage` to OpenAI `ChatCompletionRequest`
  - Converts OpenAI `ChatCompletionResponse` to `CanonicalMessage`
  - Error mapping: OpenAI errors → `SentinelError`
- `stream()`: Implements streaming via `async_openai` streaming API
- Configuration:
  - API key from `secrecy::Secret<String>` (env var: `OPENAI_API_KEY`)
  - Model selection via `config.toml`

#### 6.3.2.4 API Server (`src/api/mod.rs`)
- `create_router()` function:
  - Configures Axum router
  - Adds routes from `routes.rs`
  - Applies middleware stack
  - Returns `Router`
- `start_server(router: Router, addr: SocketAddr) -> Result<(), anyhow::Error>`:
  - Binds to address
  - Starts server with graceful shutdown
  - Logs startup/shutdown events

#### 6.3.2.5 Main Entry Point (`src/main.rs`)
- Initializes tracing subscriber
- Loads configuration (`config` crate + `dotenvy`)
- Creates OpenAI adapter
- Spawns supervisor
- Spawns initial actors
- Creates API router
- Starts HTTP server
- Handles graceful shutdown (SIGTERM/SIGINT)

### 6.3.3 Acceptance Criteria
1. External client can send HTTP POST to `/v1/chat/completions`
2. Request is validated (invalid requests return 400)
3. Response is streamed back to client (chunked transfer)
4. OpenAI adapter successfully calls OpenAI API
5. Error handling:
   - Network errors → 502
   - Invalid API key → 401
   - Rate limit → 429
6. Health check endpoint returns 200
7. Integration test: Full HTTP request → OpenAI → response cycle
8. `cargo clippy -- -D warnings` passes

### 6.3.4 Dependencies
- Phase 1 complete (Core Domain)
- Phase 2 complete (Actor System)
- OpenAI API key (test environment)

### 6.3.5 Risks
- **Risk**: OpenAI API rate limits
- **Mitigation**: Implement retry logic with exponential backoff (future phase)
- **Risk**: Streaming complexity
- **Mitigation**: Use Axum streaming utilities, test thoroughly
- **Risk**: Configuration management
- **Mitigation**: Use `config` crate, validate on startup

---

## 7. PRODUCTION PHASES (4-7)

## 7.1 PHASE 4: MEMORY MANAGEMENT SYSTEM

### 7.1.1 Objectives
Implement three-tier memory consolidation system: Short-term (in-memory), Medium-term (Sled), Long-term (Qdrant) with automatic consolidation triggers.

### 7.1.2 Deliverables

#### 7.1.2.1 Memory Manager (`src/memory/manager.rs`)
- `MemoryManager` struct:
  - `short_term: Vec<CanonicalMessage>` (in-memory, max 100 messages)
  - `sled_db: sled::Db` (medium-term storage)
  - `qdrant: Arc<dyn VectorStore>` (long-term storage)
  - `token_count: usize` (tracks current context size)
  - `consolidation_threshold: usize` (configurable, default: 10000 tokens)
- Methods:
  - `add_message(msg: CanonicalMessage) -> Result<(), SentinelError>`
  - `get_context(limit: usize) -> Result<Vec<CanonicalMessage>, SentinelError>`
  - `consolidate() -> Result<(), SentinelError>` (triggers consolidation)

#### 7.1.2.2 Short-Term Memory
- In-memory `Vec<CanonicalMessage>`
- Maximum capacity: 100 messages (configurable)
- FIFO eviction when capacity exceeded
- Token counting: Approximate via `content.len() / 4` (rough token estimate)

#### 7.1.2.3 Medium-Term Memory (Sled)
- `SledStore` struct (`src/adapters/sled.rs`):
  - Wraps `sled::Db` instance
  - Storage path: `./data/sled` (configurable)
  - Key format: `message_{uuid}` → serialized `CanonicalMessage`
  - Summary storage: `summary_{timestamp}` → consolidated summary
- Methods:
  - `store(message: CanonicalMessage) -> Result<(), SentinelError>`
  - `load(id: MessageId) -> Result<Option<CanonicalMessage>, SentinelError>`
  - `store_summary(summary: String, timestamp: DateTime<Utc>) -> Result<(), SentinelError>`
  - `load_recent_summaries(limit: usize) -> Result<Vec<String>, SentinelError>`

#### 7.1.2.4 Long-Term Memory (Qdrant)
- `QdrantRepo` struct (`src/adapters/qdrant.rs`):
  - Wraps `qdrant_client::QdrantClient`
  - Implements `VectorStore` trait
  - Collection name: `sentinel_messages` (configurable)
  - Vector dimension: 1536 (OpenAI embedding dimension)
- Embedding generation:
  - Uses OpenAI embeddings API (via `async_openai`)
  - Converts `CanonicalMessage` to embedding vector
  - Stores with metadata (id, timestamp, role)
- Search:
  - Semantic search via `VectorStore::search()`
  - Returns relevant `MessageId`s
  - Retrieves full messages from Sled

#### 7.1.2.5 Dreamer Background Task
- `dreamer_loop(memory_manager: Arc<MemoryManager>, llm: Arc<dyn LLMProvider>)`:
  - Runs every 60 seconds
  - Checks `token_count` against threshold
  - If threshold exceeded:
    1. Summarizes short-term messages via LLM
    2. Stores summary in Sled
    3. Clears short-term memory
    4. Generates embedding for summary
    5. Stores in Qdrant
  - Logs all consolidation actions via `tracing`

#### 7.1.2.6 Memory Module (`src/memory/mod.rs`)
- Export `MemoryManager`, `SledStore`, `QdrantRepo`
- Public API: `new_memory_manager(sled_path: PathBuf, qdrant: Arc<dyn VectorStore>) -> MemoryManager`

### 7.1.3 Acceptance Criteria
1. Short-term memory stores and retrieves messages correctly
2. Sled persists messages across restarts:
   - Write message to Sled
   - Restart application
   - Message retrievable after restart
3. Qdrant stores and searches embeddings:
   - Store message with embedding
   - Search returns relevant results
4. Consolidation triggers when threshold exceeded:
   - Add messages until token_count > threshold
   - Dreamer task runs and consolidates
   - Short-term memory cleared
   - Summary stored in Sled
   - Embedding stored in Qdrant
5. Integration test: Full three-tier flow (add → consolidate → search)
6. Performance: Consolidation completes within 30 seconds for 100 messages

### 7.1.4 Dependencies
- Phase 1 complete (Core Domain, VectorStore trait)
- Phase 2 complete (Actor System)
- Phase 3 complete (OpenAI Adapter)
- Qdrant instance (local or Docker)
- Sled database (local file system)

### 7.1.5 Risks
- **Risk**: Token counting accuracy
- **Mitigation**: Use approximate method, refine later if needed
- **Risk**: Sled database corruption
- **Mitigation**: Regular backups, error handling, recovery procedures
- **Risk**: Qdrant connection failures
- **Mitigation**: Retry logic, fallback to Sled-only mode

---

## 7.2 PHASE 5: OBSERVABILITY & TELEMETRY

### 7.2.1 Objectives
Implement comprehensive observability stack: structured logging, distributed tracing, metrics collection, and health monitoring.

### 7.2.2 Deliverables

#### 7.2.2.1 Tracing Setup (`src/telemetry/mod.rs`)
- `init_tracing(service_name: &str) -> Result<(), anyhow::Error>`:
  - Configures `tracing_subscriber` with:
    - `env_filter` (RUST_LOG environment variable)
    - `json` format for structured logging
    - File output: `logs/sentinel.json` (rotating, 100MB max)
    - Console output: Human-readable format
  - Sets log level from environment: `RUST_LOG=info, sentinel=debug`
- Span creation:
  - All actor loops create spans: `tracing::span!("actor_loop", actor_id = %id)`
  - All HTTP requests create spans: `tracing::span!("http_request", path = %path, method = %method)`
  - Memory operations create spans: `tracing::span!("memory_operation", operation = "consolidate")`

#### 7.2.2.2 OpenTelemetry Integration
- Metrics collection:
  - Counter: `messages_processed_total` (labels: `status`, `actor_id`)
  - Histogram: `message_processing_duration_seconds` (labels: `actor_id`)
  - Gauge: `active_actors` (current number of actors)
  - Gauge: `memory_token_count` (current token count)
- Export format: Prometheus-compatible
- Endpoint: `GET /metrics` (Prometheus scrape endpoint)

#### 7.2.2.3 Structured Logging
- All log events include:
  - Timestamp (ISO 8601)
  - Level (error, warn, info, debug, trace)
  - Target (module path)
  - Fields (structured key-value pairs)
- Critical events logged:
  - Actor spawn/termination
  - State transitions
  - Memory consolidation
  - Errors with full context
  - HTTP requests/responses

#### 7.2.2.4 Health Check Endpoints
- `GET /health`:
  - Returns: `{"status": "healthy", "timestamp": "2025-01-20T10:00:00Z"}`
  - Checks: Database connectivity (Sled), Qdrant connectivity
- `GET /health/ready`:
  - Returns: `{"status": "ready"}` if all components initialized
- `GET /health/live`:
  - Returns: `{"status": "alive"}` (always 200, for Kubernetes liveness)

#### 7.2.2.5 Telemetry Module (`src/telemetry/mod.rs`)
- Export: `init_tracing`, `record_metric`, `create_span`
- Public API: `init_telemetry(service_name: &str, log_level: &str) -> Result<(), anyhow::Error>`

### 7.2.3 Acceptance Criteria
1. All log events output in JSON format to file
2. Console output is human-readable
3. Tracing spans create parent-child relationships:
   - HTTP request span → Actor span → LLM call span
4. Metrics endpoint returns Prometheus format:
   - `GET /metrics` returns valid Prometheus exposition format
5. Health checks return correct status codes:
   - Healthy system: 200
   - Unhealthy system: 503
6. Integration test: Verify logs contain expected fields
7. Performance: Logging overhead < 1% of request processing time

### 7.2.4 Dependencies
- Phase 1 complete (Core Domain)
- Phase 2 complete (Actor System)
- Phase 3 complete (API Gateway)
- Phase 4 complete (Memory Management)

### 7.2.5 Risks
- **Risk**: Log volume overwhelming disk
- **Mitigation**: Log rotation, configurable retention, alerting
- **Risk**: Tracing overhead
- **Mitigation**: Sample rate configuration, async span creation
- **Risk**: Metrics cardinality explosion
- **Mitigation**: Limit label cardinality, aggregate appropriately

---

## 7.3 PHASE 6: RESILIENCE & PRODUCTION HARDENING

### 7.3.1 Objectives
Implement production-grade resilience features: circuit breakers, rate limiting, backpressure handling, and graceful degradation.

### 7.3.2 Deliverables

#### 7.3.2.1 Circuit Breakers (Tower)
- `CircuitBreaker` middleware:
  - Wraps LLM provider calls
  - State: `Closed` (normal), `Open` (failing), `HalfOpen` (testing)
  - Failure threshold: 5 consecutive failures → `Open`
  - Success threshold: 1 success → `Closed`
  - Timeout: 30 seconds in `Open` state before `HalfOpen`
- Implementation: Use `tower::ServiceBuilder` with custom circuit breaker layer
- Error handling: Circuit breaker open returns `503 Service Unavailable`

#### 7.3.2.2 Rate Limiting (`tower-http`)
- Global rate limiter:
  - Requests per minute: 100 (configurable)
  - Per-IP rate limiter: 10 requests/minute (configurable)
- Implementation: `tower_http::limit::RateLimitLayer`
- Response: `429 Too Many Requests` with `Retry-After` header

#### 7.3.2.3 Budget Kill Switches
- `BudgetTracker` struct:
  - Tracks OpenAI API costs (estimated)
  - Daily budget limit: $100 (configurable)
  - Monthly budget limit: $2000 (configurable)
- Kill switch behavior:
  - When budget exceeded, reject new requests with `503`
  - Log budget exhaustion event
  - Alert via metrics (gauge: `budget_exceeded`)

#### 7.3.2.4 Backpressure Handling
- Bounded channels:
  - Actor input channel: Size 32 (configurable)
  - API request channel: Size 64 (configurable)
  - When channel full, reject request with `503`
- Timeout handling:
  - All channel sends have 5-second timeout
  - Timeout returns `504 Gateway Timeout`

#### 7.3.2.5 Configuration Management
- `Config` struct (via `config` crate):
  - Load from `config.toml`
  - Override with environment variables
  - Validate on startup
  - Fields:
    - `server.port: u16`
    - `server.host: String`
    - `openai.model: String`
    - `openai.api_key: Secret<String>`
    - `memory.consolidation_threshold: usize`
    - `memory.sled_path: String`
    - `qdrant.url: String`
    - `rate_limit.requests_per_minute: u64`
    - `budget.daily_limit: f64`
    - `budget.monthly_limit: f64`

#### 7.3.2.6 Graceful Shutdown
- Signal handling:
  - Listen for `SIGTERM`, `SIGINT`
  - Initiate graceful shutdown sequence:
    1. Stop accepting new requests (return 503)
    2. Wait for in-flight requests to complete (max 30s)
    3. Close all channels
    4. Flush logs
    5. Close database connections
    6. Exit

### 7.3.3 Acceptance Criteria
1. Circuit breaker opens after 5 consecutive failures
2. Circuit breaker recovers after 30 seconds + 1 success
3. Rate limiter rejects requests exceeding limit (429)
4. Budget kill switch triggers when limit exceeded (503)
5. Backpressure: Full channels reject requests (503)
6. Configuration loads from file and environment variables
7. Graceful shutdown completes within 30 seconds
8. Integration test: Simulate overload, verify graceful handling

### 7.3.4 Dependencies
- Phase 1 complete (Core Domain)
- Phase 2 complete (Actor System)
- Phase 3 complete (API Gateway)
- Phase 4 complete (Memory Management)
- Phase 5 complete (Observability)

### 7.3.5 Risks
- **Risk**: Circuit breaker false positives
- **Mitigation**: Tune thresholds, monitor metrics, alert on state changes
- **Risk**: Rate limiting too aggressive
- **Mitigation**: Make limits configurable, monitor rejection rates
- **Risk**: Budget tracking inaccuracy
- **Mitigation**: Use conservative estimates, add buffer, alert early

---

## 7.4 PHASE 7: ADVANCED FEATURES & OPTIMIZATION

### 7.4.1 Objectives
Implement advanced features for production: deterministic replay, interrupt persistence, multi-agent coordination, and performance optimization.

### 7.4.2 Deliverables

#### 7.4.2.1 Deterministic Replay Capability
- `ReplayEngine` struct:
  - Records all actor state transitions to Sled
  - Replay format: `replay_{actor_id}_{timestamp}` → serialized state history
  - Replay method: `replay(actor_id: ActorId, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<StateTransition>, SentinelError>`
- Use cases:
  - Debugging production issues
  - Audit trail
  - Testing with historical data

#### 7.4.2.2 Interrupt Persistence
- `InterruptStore` struct:
  - Persists actor state to Sled on interrupt signal
  - Resume method: `resume(actor_id: ActorId) -> Result<ActorHandle, SentinelError>`
  - State includes:
    - Current `AgentState`
    - Pending messages
    - Context from memory manager
- Interrupt triggers:
  - Graceful shutdown
  - Manual interrupt via API endpoint
  - Error recovery

#### 7.4.2.3 Multi-Agent Coordination
- `CoordinationManager` struct:
  - Manages multiple agents with shared context
  - Agent registration: `register_agent(id: ActorId, specialization: String) -> Result<(), SentinelError>`
  - Message routing: Routes messages to appropriate agent based on specialization
  - Shared memory: Agents access shared memory context
- Coordination patterns:
  - Parallel execution: Spawn multiple agents, aggregate results
  - Sequential pipeline: Chain agents, pass output as input
  - Broadcast: Send message to all agents, collect responses

#### 7.4.2.4 Performance Optimization
- Connection pooling:
  - Reqwest client with connection pool (max 100 connections)
  - Qdrant client connection reuse
  - Sled batch writes (buffer writes, flush periodically)
- Caching:
  - Embedding cache: Store embeddings in-memory (LRU, max 1000)
  - Message cache: Cache frequently accessed messages
- Parallel processing:
  - Parallel LLM calls when multiple agents active
  - Parallel memory consolidation (batch operations)
- Profiling:
  - Benchmark suite: `cargo bench` with `criterion`
  - Performance targets:
    - Message processing: < 100ms p99
    - Memory consolidation: < 5s for 100 messages
    - API response time: < 200ms p99

#### 7.4.2.5 Production Deployment Documentation
- Deployment guide:
  - Docker image build instructions
  - Kubernetes manifests (Deployment, Service, ConfigMap)
  - Environment variables reference
  - Health check configuration
- Operations guide:
  - Monitoring dashboard setup (Grafana)
  - Alerting rules (Prometheus)
  - Log aggregation (Loki/ELK)
  - Backup procedures (Sled, Qdrant)
- Runbook:
  - Common issues and resolutions
  - Rollback procedures
  - Disaster recovery
  - Performance tuning

### 7.4.3 Acceptance Criteria
1. Replay engine can replay actor state from any point in time
2. Interrupt persistence: Actor state survives restart
3. Multi-agent coordination: 10 agents process messages in parallel
4. Performance targets met:
   - Message processing: < 100ms p99
   - API response: < 200ms p99
   - Memory consolidation: < 5s
5. Connection pooling reduces connection overhead by 50%
6. Caching reduces redundant API calls by 30%
7. Deployment documentation complete and tested
8. Integration test: Full production scenario (100 concurrent requests, 10 agents)

### 7.4.4 Dependencies
- Phase 1-6 complete (All previous phases)
- Production infrastructure (Kubernetes, monitoring)

### 7.4.5 Risks
- **Risk**: Replay complexity
- **Mitigation**: Start simple, add features incrementally, test thoroughly
- **Risk**: Interrupt persistence data loss
- **Mitigation**: Atomic writes, checksums, verification on resume
- **Risk**: Multi-agent coordination overhead
- **Mitigation**: Profile and optimize, use efficient data structures
- **Risk**: Performance regressions
- **Mitigation**: Continuous benchmarking, performance tests in CI

---

## 8. SUCCESS METRICS

### 8.1 Performance Metrics
- **Throughput**: 1000+ messages/second (p95)
- **Latency**: < 200ms API response time (p99)
- **Memory**: < 2GB RAM usage under normal load
- **CPU**: < 50% CPU usage under normal load

### 8.2 Reliability Metrics
- **Uptime**: 99.9% availability
- **Error Rate**: < 0.1% of requests result in 5xx errors
- **Recovery Time**: < 30 seconds for automatic recovery

### 8.3 Quality Metrics
- **Test Coverage**: > 80% code coverage
- **Lint**: Zero `cargo clippy` warnings
- **Documentation**: All public APIs documented

### 8.4 Operational Metrics
- **Deployment Time**: < 5 minutes for zero-downtime deployment
- **Alert Response**: < 5 minutes to acknowledge critical alerts
- **Incident Resolution**: < 1 hour for critical incidents

---

## 9. RISK ASSESSMENT

### 9.1 Technical Risks

#### 9.1.1 High Risk
- **LLM Provider API Changes**: OpenAI API changes could break adapter
  - **Probability**: Medium
  - **Impact**: High
  - **Mitigation**: Trait abstraction allows quick adapter replacement, version pinning

- **Memory Consolidation Accuracy**: Summarization may lose critical information
  - **Probability**: Medium
  - **Impact**: High
  - **Mitigation**: Tune summarization prompts, verify with tests, allow manual override

#### 9.1.2 Medium Risk
- **Performance Under Load**: System may not meet throughput targets
  - **Probability**: Medium
  - **Impact**: Medium
  - **Mitigation**: Load testing, profiling, optimization in Phase 7

- **Database Corruption**: Sled or Qdrant corruption could cause data loss
  - **Probability**: Low
  - **Impact**: High
  - **Mitigation**: Regular backups, error handling, recovery procedures

#### 9.1.3 Low Risk
- **Dependency Updates**: Breaking changes in dependencies
  - **Probability**: Low
  - **Impact**: Medium
  - **Mitigation**: Version pinning, dependency update process, testing

### 9.2 Operational Risks

#### 9.2.1 High Risk
- **API Key Leakage**: Exposure of OpenAI API keys
  - **Probability**: Low
  - **Impact**: High
  - **Mitigation**: Use `secrecy` crate, environment variables, key rotation

#### 9.2.2 Medium Risk
- **Resource Exhaustion**: Out of memory or disk space
  - **Probability**: Medium
  - **Impact**: Medium
  - **Mitigation**: Resource limits, monitoring, alerting, automatic cleanup

### 9.3 Business Risks
- **Cost Overruns**: OpenAI API costs exceed budget
  - **Probability**: Medium
  - **Impact**: Medium
  - **Mitigation**: Budget kill switches (Phase 6), cost monitoring, alerts

---

## 10. DEPENDENCIES & BLOCKERS

### 10.1 External Dependencies
- **OpenAI API**: Required for LLM functionality (Phase 3+)
- **Qdrant Instance**: Required for long-term memory (Phase 4+)
- **Rust Toolchain**: Rust 1.70+ required
- **Docker**: Required for integration tests

### 10.2 Internal Dependencies
- Phase dependencies are sequential (1 → 2 → 3 → 4 → 5 → 6 → 7)
- Each phase must meet acceptance criteria before proceeding
- No parallel development of dependent phases

### 10.3 Blockers
- **None identified at this time**

### 10.4 Critical Path
1. Phase 1 (Foundation) → Phase 2 (Actor System) → Phase 3 (API/Adapter)
2. Phase 3 → Phase 4 (Memory) → Phase 5 (Observability)
3. Phase 5 → Phase 6 (Resilience) → Phase 7 (Advanced Features)

---

## 11. TIMELINE ESTIMATE

### 11.1 MVP Phases (1-3)
- **Phase 1**: 1-2 weeks
- **Phase 2**: 2-3 weeks
- **Phase 3**: 2-3 weeks
- **Total MVP**: 5-8 weeks

### 11.2 Production Phases (4-7)
- **Phase 4**: 3-4 weeks
- **Phase 5**: 2-3 weeks
- **Phase 6**: 2-3 weeks
- **Phase 7**: 3-4 weeks
- **Total Production**: 10-14 weeks

### 11.3 Total Timeline
- **MVP Delivery**: 5-8 weeks
- **Production Delivery**: 15-22 weeks (including MVP)

### 11.4 Assumptions
- Single developer full-time
- No external blockers
- Standard development velocity
- Adequate testing infrastructure available

---

## 12. APPENDIX

### 12.1 Glossary
- **CanonicalMessage**: Pure domain type representing a message, no external dependencies
- **Hexagonal Architecture**: Architecture pattern separating domain from infrastructure
- **Actor**: Independent processing unit communicating via message passing
- **State Machine**: Explicit state transitions with validation
- **Three-Tier Memory**: Short-term (RAM), Medium-term (Sled), Long-term (Qdrant)

### 12.2 References
- Architecture Documentation: `docs/architecture.md`
- Research Comparison: `docs/research/rust_orchestrators_comparison.md`
- Agent Roles: `AGENTS.md`
- Cursor Rules: `.cursor/rules/*.mdc`

### 12.3 Document History
- Version 1.0 (2025-01-20): Initial PRD

---

**END OF DOCUMENT**

