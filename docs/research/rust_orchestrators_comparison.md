# Rust Orchestrators vs Sentinel PRD: Comprehensive Comparison

## Executive Summary

This document compares existing Rust-based orchestrator frameworks against the Sentinel Orchestrator PRD requirements. The analysis identifies patterns worth adopting, unique Sentinel requirements, and gaps in existing solutions.

## Rust Orchestrator Frameworks

### 1. Swarms-rs

**Repository**: `The-Swarm-Corporation/swarms-rs`

**Type**: Enterprise-grade, production-ready multi-agent orchestration framework

**Key Features**:
- Extreme performance leveraging Rust's zero-cost abstractions and concurrency
- Modular design enabling scalability to thousands of agents
- Memory safety through Rust's ownership model
- Production-grade reliability patterns

**Architecture Patterns**:
- Actor-based message passing
- Modular plugin system
- Performance-first design

**Alignment with Sentinel PRD**: **High**
- ✅ Matches emphasis on performance and modularity
- ✅ Supports large-scale agent orchestration
- ⚠️ Less strict about hexagonal architecture boundaries

---

### 2. Swarm (fcn06/swarm)

**Repository**: `fcn06/swarm`

**Type**: Framework for building and managing networks of intelligent, self-correcting agent teams

**Key Features**:
- Self-correcting workflows with automated planning
- Dynamic scaling via Agent Factory pattern
- Open standards focus (MCP, A2A protocols)
- Flexible workflow definitions

**Architecture Patterns**:
- Agent Factory for runtime instantiation
- Self-correction mechanisms
- Protocol-based agent communication

**Alignment with Sentinel PRD**: **Medium-High**
- ✅ Agent lifecycle management patterns align with Supervisor role
- ✅ Dynamic agent creation useful for Sentinel
- ⚠️ Less emphasis on canonical message models
- ⚠️ Workflow-focused rather than actor-focused

---

### 3. Orka

**Repository**: `excsn/orka`

**Type**: Asynchronous, pluggable, type-safe workflow engine

**Key Features**:
- Explicit pipeline definitions (first-class pipelines)
- Decoupled step logic for modularity
- Managed shared state with safe concurrent access
- Conditional branching and control flow

**Architecture Patterns**:
- Pipeline-as-code definitions
- Type-safe state management
- Pluggable step execution

**Alignment with Sentinel PRD**: **Medium**
- ✅ Type safety aligns with Sentinel's strict typing requirements
- ✅ Modular design principles
- ⚠️ Workflow-oriented rather than agent-oriented
- ⚠️ Less relevant to actor model patterns

---

### 4. ccswarm

**Repository**: `nwiizo/ccswarm`

**Type**: High-performance multi-agent orchestration system

**Key Features**:
- Rust-native patterns (zero-cost abstractions)
- Type-state patterns for compile-time safety
- Channel-based communication (similar to Sentinel's approach)
- Claude Code integration via Agent Client Protocol (ACP)
- Sangha Collective Intelligence System

**Architecture Patterns**:
- Channel-based message passing
- Zero-cost abstractions
- Protocol-based agent communication

**Alignment with Sentinel PRD**: **High**
- ✅ Channel-based communication matches `tokio::sync::mpsc` approach
- ✅ Zero-cost abstractions align with performance goals
- ✅ Multi-agent coordination patterns relevant
- ⚠️ Less strict about domain model boundaries

---

### 5. Sentinel (raskell.io/sentinel)

**Note**: This is a different project - a security-focused reverse proxy, not an agent orchestrator. Included for completeness but not directly relevant to our use case.

---

## Sentinel PRD Requirements Analysis

### Architecture Requirements

#### Hexagonal Architecture (Ports & Adapters)

**Requirement**: Strict separation between domain (`src/core/`) and infrastructure (`src/adapters/`)

**Comparison**:
- **Swarms-rs**: Modular but not strictly hexagonal
- **Swarm**: Protocol-based, less strict boundaries
- **Orka**: Pipeline-based, different paradigm
- **ccswarm**: Channel-based, not explicitly hexagonal
- **Sentinel Gap**: None enforce strict hexagonal architecture to the degree required

**Verdict**: Sentinel must implement this uniquely; no direct reference implementation found.

#### Canonical Message Model

**Requirement**: `CanonicalMessage` as pure domain type, no external dependencies

**Comparison**:
- Most frameworks allow external types in domain models
- Few enforce "canonical" message types
- Protocol-based approaches use external protocol definitions

**Verdict**: Unique Sentinel requirement; must maintain strict domain model purity.

#### State Machine Orchestration

**Requirement**: Explicit state transitions (`Idle -> Thinking -> ToolCall -> Reflecting`)

**Comparison**:
- **Orka**: Has workflow states but more implicit
- **Swarm**: Workflow-oriented, not state machine
- Most frameworks use implicit state via message passing

**Verdict**: Sentinel needs explicit state machine; can reference workflow state patterns from Orka.

---

### Technical Requirements

#### Message Passing Architecture

**Requirement**: `tokio::sync::mpsc` channels, avoid `Arc<Mutex<T>>`

**Comparison**:
- **ccswarm**: ✅ Uses channel-based communication
- **Swarms-rs**: Uses actor model with messages
- **Swarm**: Protocol-based, similar patterns
- Most avoid `Arc<Mutex>` in favor of message passing

**Verdict**: Well-established pattern; ccswarm provides good reference.

#### Async Patterns

**Requirement**: `tokio::select!` for cancellation safety

**Comparison**:
- Standard Tokio pattern used across frameworks
- All modern Rust async orchestrators use `tokio::select!`

**Verdict**: Industry standard; well-documented in Tokio ecosystem.

#### Error Handling

**Requirement**: `thiserror` in core, `anyhow` in application code

**Comparison**:
- Most frameworks use `thiserror` or similar
- Standard Rust error handling patterns
- Sentinel's split (core vs app) is more strict than most

**Verdict**: Standard pattern; Sentinel's strict separation is unique.

---

### Agent Roles Requirements

#### 1. The Sentinel (Orchestrator)

**Role**: Central coordination actor managing event loop and state transitions

**Comparison**:
- **Swarms-rs**: Has orchestrator pattern but less explicit state machine
- **ccswarm**: Similar coordinator patterns
- **Swarm**: Workflow orchestrator exists but different model

**Patterns to Adopt**:
- Event loop patterns from Swarms-rs
- Coordination logic from ccswarm

#### 2. The Supervisor

**Role**: Process manager, zombie detection (>60s timeout)

**Comparison**:
- **Swarm**: Agent Factory provides lifecycle management
- Most frameworks have supervisor patterns
- Timeout-based health checks are standard

**Patterns to Adopt**:
- Agent Factory pattern from Swarm for dynamic agent creation
- Health check patterns from Swarms-rs

#### 3. The Dreamer (Memory Manager)

**Role**: Background optimization, memory consolidation (Short → Medium → Long-term)

**Comparison**:
- **Unique to Sentinel**: No framework implements three-tier memory consolidation
- Most use single storage or simple caching
- Memory summarization patterns exist but not integrated

**Verdict**: Unique requirement; must implement from scratch with reference to:
- Token counting patterns
- Summarization strategies
- Background task patterns

---

### Integration Requirements

#### LLM Provider Trait

**Requirement**: Trait-based abstraction in core, implementations in adapters

**Comparison**:
- Standard Rust trait pattern
- Most frameworks have provider abstractions
- Sentinel's strict separation (no external types in core) is unique

**Pattern to Adopt**: Standard trait pattern; maintain strict boundaries.

#### Vector Store Trait

**Requirement**: Trait-based abstraction for Qdrant

**Comparison**:
- Standard abstraction pattern
- Many frameworks abstract storage
- Sentinel's canonical message requirement adds complexity

**Pattern to Adopt**: Standard trait pattern.

#### Memory Tiering

**Requirement**: Short-Term (in-memory) → Medium-Term (Sled) → Long-Term (Qdrant)

**Comparison**:
- **Unique**: No framework implements this three-tier system
- Some have caching layers but not explicit consolidation

**Verdict**: Must implement uniquely; reference:
- Sled usage patterns
- Qdrant integration patterns
- Token counting for consolidation triggers

---

## Design Pattern Recommendations

### Patterns to Adopt

1. **Channel-Based Communication** (from ccswarm)
   - Use `tokio::sync::mpsc` for all actor communication
   - Avoid shared mutable state
   - Match Sentinel's requirement exactly

2. **Agent Factory Pattern** (from Swarm)
   - Dynamic agent creation for Supervisor
   - Runtime instantiation patterns
   - Lifecycle management

3. **Performance Patterns** (from Swarms-rs)
   - Zero-cost abstractions
   - Async task spawning strategies
   - Resource pooling patterns

4. **Type Safety Patterns** (from Orka)
   - Type-state patterns where applicable
   - NewType patterns for IDs
   - Compile-time guarantees

### Patterns to Avoid

1. **External Types in Domain**
   - Don't allow `async_openai` types in `src/core/`
   - Maintain canonical message model
   - Map at adapter boundaries

2. **Shared Mutable State**
   - Avoid `Arc<Mutex<T>>` patterns
   - Prefer message passing
   - Use channels for coordination

3. **Implicit State Machines**
   - Don't rely on implicit state via message passing
   - Make state transitions explicit
   - Document state machine clearly

---

## Gaps and Unique Requirements

### Gaps in Existing Frameworks

1. **Strict Hexagonal Architecture**
   - None enforce to Sentinel's degree
   - Most allow some infrastructure leakage

2. **Three-Tier Memory System**
   - No framework implements Short/Medium/Long-term consolidation
   - Must build from scratch

3. **Canonical Message Model**
   - Most allow external types in domain
   - Sentinel's strict separation is unique

4. **Cursor 2.0 Integration**
   - None target Cursor specifically
   - Must design integration points

### Unique Sentinel Requirements

1. **Explicit State Machine**: More rigid than workflow-based approaches
2. **Memory Consolidation**: Unique three-tier system with summarization
3. **Domain Model Purity**: Stricter than most frameworks
4. **Integration Specificity**: Cursor 2.0 requirements unique

---

## Implementation Recommendations

### Phase 1: Core Domain
- Study Swarms-rs for performance patterns
- Implement strict hexagonal boundaries
- Create canonical message model (no external deps)

### Phase 2: Actor System
- Use ccswarm patterns for channel communication
- Implement explicit state machine
- Reference Swarm for agent lifecycle patterns

### Phase 3: Memory System
- Design from scratch (no direct reference)
- Reference summarization techniques
- Implement token counting for triggers

### Phase 4: Integration
- Standard trait patterns for LLM/VectorStore
- Cursor 2.0 integration (unique requirement)
- Maintain strict adapter boundaries

---

## References

- [Swarms-rs](https://github.com/The-Swarm-Corporation/swarms-rs)
- [Swarm](https://github.com/fcn06/swarm)
- [Orka](https://github.com/excsn/orka)
- [ccswarm](https://github.com/nwiizo/ccswarm)
- [Tokio Documentation](https://tokio.rs/)
- [Rust Async Patterns](https://rust-lang.github.io/async-book/)

---

## Last Updated

Research conducted: Current date
Main branch sync: Periodic pulls recommended (see `docs/MAIN_BRANCH_SYNC.md`)

