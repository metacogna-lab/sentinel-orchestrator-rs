# Sentinel Orchestrator - Development Start Guide

## Overview

This document provides the authoritative development workflow and rules for building the Sentinel Orchestrator. **Follow this guide strictly** - it enforces all project rules and ensures consistent, high-quality development.

## Core Principles

1. **Backend First**: Complete ALL backend/Rust work before touching the frontend
2. **Test-Driven**: Write tests first, then implement, then verify
3. **Branch Per Feature**: Each numbered work item gets its own feature branch
4. **Canonical Model**: All communication uses `CanonicalMessage` from `src/core/types.rs`
5. **OpenAPI Schema**: Frontend must follow `openapi.yaml` schema (generate from backend)
6. **Edit Only**: Never delete existing code, only edit and extend
7. **Error Tracking**: Record all errors in `tasks/errors.md`

## Development Workflow

### For Each Numbered Work Item:

1. **Create Feature Branch**
   ```bash
   git checkout main
   git pull origin main
   git checkout -b feature/phase-X-item-Y
   ```

2. **Write Tests First**
   - Unit tests with `mockall` for traits
   - Integration tests in `tests/` directory
   - Use `#[tokio::test]` for async tests
   - Test error paths, not just happy paths

3. **Implement Feature**
   - Follow `@tasks/prd.md` for requirements
   - Follow `@tasks/instructions.md` for workflow
   - Follow `CLAUDE.md` for Rust standards
   - Run `cargo check` frequently (or `/check` in Claude Code)
   - Use `rs_agent` in Claude Code for Rust work

4. **Test Implementation**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Commit and Push**
   ```bash
   git add .
   git commit -m "feat: [description of work item]"
   git push origin feature/phase-X-item-Y
   ```

6. **Merge to Main**
   ```bash
   git checkout main
   git merge feature/phase-X-item-Y
   git push origin main
   ```

7. **Continue to Next Item**
   - Update `tasks/bridge.md` with current/next state
   - Start new branch for next numbered item
   - Repeat workflow

## Backend Development Rules (Rust)

### Phase 1: Core Domain (MUST COMPLETE FIRST)

**Location**: `src/core/`

**CRITICAL RULES**:
- **ZERO external I/O dependencies** in `src/core/`
- Only allowed dependencies: `serde`, `uuid`, `thiserror`, `chrono`
- **FORBIDDEN**: `tokio`, `reqwest`, `axum`, `qdrant`, `openai`, or any infrastructure crates
- All types must be pure Rust with no external types

**Work Items**:
1. ✅ Domain types (`CanonicalMessage`, `AgentState`, `Role`, `MessageId`, `AgentId`)
2. Core traits (`LLMProvider`, `VectorStore`, `MemoryStore`) in `src/core/traits.rs`
3. Domain errors (`SentinelError`) in `src/core/error.rs` using `thiserror`
4. State machine validation logic
5. Canonical message validation and conversion helpers

### Phase 2: Adapters (Infrastructure Layer)

**Location**: `src/adapters/`

**CRITICAL RULES**:
- Implement traits from `src/core/traits.rs`
- Map external types to `CanonicalMessage` at boundaries
- Use `anyhow` for error context (not `thiserror`)
- Handle all I/O errors and convert to domain errors

**Work Items**:
1. OpenAI adapter (`src/adapters/openai.rs`)
   - Implement `LLMProvider` trait
   - Map `async_openai` types → `CanonicalMessage`
   - Map `CanonicalMessage` → `async_openai` types
2. Qdrant adapter (`src/adapters/qdrant.rs`)
   - Implement `VectorStore` trait
   - Map Qdrant types → `CanonicalMessage`
3. Sled adapter (`src/adapters/sled.rs`)
   - Implement `MemoryStore` trait
   - Medium-term memory storage

### Phase 3: Engine (Actor System)

**Location**: `src/engine/`

**CRITICAL RULES**:
- Use `tokio::sync::mpsc::channel(N)` for bounded channels (never unbounded)
- No `Arc<Mutex<T>>` - use channels instead
- Use `tokio::select!` for cancellation and timeouts
- Explicit state machine transitions

**Work Items**:
1. Actor event loop (`src/engine/actor.rs`)
   - Main event loop with `tokio::select!`
   - State machine transitions
   - Message processing
2. Supervisor (`src/engine/supervisor.rs`)
   - Agent lifecycle management
   - Zombie detection (>60s timeout)
   - Health monitoring

### Phase 4: Memory System

**Location**: `src/memory/`

**Work Items**:
1. Memory manager (`src/memory/manager.rs`)
   - Three-tier memory coordination
   - Token counting for consolidation triggers
   - Short-term → Medium-term → Long-term flow

### Phase 5: API Layer

**Location**: `src/api/`

**CRITICAL RULES**:
- HTTP routes use Axum
- Convert JSON DTOs → `CanonicalMessage`
- Convert `CanonicalMessage` → JSON responses
- Generate OpenAPI schema from routes

**Work Items**:
1. Route handlers (`src/api/routes.rs`)
   - Health check endpoint
   - Chat completion endpoint
   - Agent status endpoint
2. Middleware (`src/api/middleware.rs`)
   - Request logging
   - Error handling
   - CORS configuration
3. **Generate OpenAPI Schema**
   - Use `utoipa` or similar to generate `openapi.yaml`
   - Ensure schema matches `CanonicalMessage` types
   - Commit `openapi.yaml` to repository

### Phase 6: Integration & Testing

**Work Items**:
1. Integration tests in `tests/` directory
2. End-to-end API tests
3. Performance benchmarks
4. Documentation

## Frontend Development Rules (TypeScript)

### ⚠️ CRITICAL: DO NOT START UNTIL BACKEND IS COMPLETE

**Only begin frontend work when**:
- ✅ All backend phases (1-6) are complete
- ✅ `openapi.yaml` schema is generated and committed
- ✅ Backend API is fully tested and working
- ✅ All backend tests pass

### Frontend Requirements

**Location**: `frontend/`

**CRITICAL RULES**:
- TypeScript types MUST match `CanonicalMessage` from backend
- API calls MUST follow `openapi.yaml` schema
- Use generated TypeScript types from OpenAPI schema (if available)
- Never create types that don't match backend `CanonicalMessage`

**Work Items**:
1. Generate TypeScript types from `openapi.yaml`
2. API client implementation
3. UI components matching canonical data model
4. State management
5. Integration with backend API

## Error Handling Rules

### Backend (Rust)

**Core Layer** (`src/core/`):
- Use `thiserror` for domain errors
- Define `SentinelError` enum
- No context, just error types

**Application Layer** (`src/api/`, `src/engine/`):
- Use `anyhow` for error context
- Convert `SentinelError` to HTTP responses
- Add context with `.context("Failed to...")?`

**FORBIDDEN**:
- `unwrap()` or `expect()` in production code
- Silently swallowing errors (always propagate with `?`)
- Panicking in library code

### Error Tracking

**Always record errors in `tasks/errors.md`**:
- Compilation errors
- Test failures
- Runtime errors
- Design issues
- Include date, description, and resolution

## Testing Requirements

### Unit Tests
- Use `mockall` for trait mocking
- Co-locate tests with code in `mod tests`
- Test error paths, not just happy paths

### Integration Tests
- In `tests/` directory
- Use Docker for external dependencies (Qdrant, etc.)
- Don't mock storage in integration tests

### Test Commands
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo test
```

## Code Quality Checks

### Before Every Commit

```bash
# Check compilation
cargo check

# Run linter (must pass with no warnings)
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check

# Run all tests
cargo test
```

### If Using Claude Code

- Run `/check` command frequently
- Use `rs_agent` for Rust-specific assistance
- Follow all `.cursor/rules/*.mdc` guidelines

## Hexagonal Architecture Enforcement

### Core Layer (`src/core/`)
- **MUST NOT** import: `tokio`, `reqwest`, `axum`, `qdrant`, `openai`
- **MUST** contain only: domain types, traits, errors
- **MUST** be pure Rust with no external I/O

### Adapters Layer (`src/adapters/`)
- **MUST** implement traits from `src/core/traits.rs`
- **MUST** map external types to `CanonicalMessage`
- **MUST** handle all I/O and convert errors

### Boundary Rules
- API receives JSON → DTO → `CanonicalMessage`
- `CanonicalMessage` → External types (in adapter)
- External types → `CanonicalMessage` (in adapter)

## Canonical Data Model

**Source of Truth**: `src/core/types.rs`

**Key Types**:
- `CanonicalMessage` - All message communication
- `AgentState` - State machine states
- `Role` - Message roles (User, Assistant, System)
- `MessageId` - Message identifier (NewType around Uuid)
- `AgentId` - Agent identifier (NewType around Uuid)

**Frontend MUST**:
- Use exact same field names
- Use exact same enum values
- Match serialization format (lowercase enums, etc.)

## OpenAPI Schema Requirements

**Location**: `openapi.yaml` (root directory)

**Requirements**:
- Generated from backend API routes
- Must match `CanonicalMessage` structure exactly
- Must include all API endpoints
- Must include request/response schemas
- Must be committed to repository

**Frontend MUST**:
- Use this schema for API client generation
- Never deviate from schema definitions
- Report schema mismatches in `tasks/errors.md`

## State Management

**Update `tasks/bridge.md`**:
- **Current State**: What you're working on now
- **Next State**: What comes next

**Update `tasks/errors.md`**:
- Record all errors encountered
- Include resolution steps
- Date each entry

## Branch Naming Convention

Format: `feature/phase-X-item-Y-description`

Examples:
- `feature/phase-1-item-2-core-traits`
- `feature/phase-2-item-1-openai-adapter`
- `feature/phase-5-item-3-openapi-schema`

## Commit Message Format

Format: `feat: [phase X, item Y] description`

Examples:
- `feat: [phase 1, item 2] implement core traits`
- `feat: [phase 2, item 1] OpenAI adapter with CanonicalMessage mapping`
- `fix: [phase 3, item 1] actor event loop cancellation`

## Prohibited Practices

### Code
- ❌ `unwrap()` or `expect()` in production code
- ❌ Unbounded channels (`mpsc::unbounded_channel`)
- ❌ `Arc<Mutex<T>>` for shared state (use channels)
- ❌ External types in `src/core/`
- ❌ Blocking I/O in async code
- ❌ Deleting existing code (only edit/extend)

### Workflow
- ❌ Starting frontend before backend is complete
- ❌ Skipping tests
- ❌ Committing without `cargo clippy` passing
- ❌ Working on multiple items in one branch
- ❌ Merging without tests passing

## References

- **PRD**: `@tasks/prd.md` - Main requirements document
- **Instructions**: `@tasks/instructions.md` - Workflow guide
- **Architecture**: `docs/architecture.md` - System architecture
- **CLAUDE.md**: Rust standards and patterns
- **Bridge**: `@tasks/bridge.md` - Current/next state
- **Errors**: `@tasks/errors.md` - Error tracking

## Getting Started

1. Read this entire document
2. Read `@tasks/prd.md` to understand requirements
3. Check `@tasks/bridge.md` for current state
4. Check `@tasks/errors.md` for known issues
5. Start with Phase 1, Item 1 (if not already done)
6. Follow the workflow: branch → test → implement → test → commit → merge → next

## Questions or Issues?

- Record in `tasks/errors.md`
- Update `tasks/bridge.md` with blockers
- Follow existing patterns in codebase
- Refer to `CLAUDE.md` for Rust-specific questions

---

**Remember**: Backend first, test-driven, one item per branch, canonical model always, edit only never delete.

