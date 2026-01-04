# Task 12: Adapter Boundary Verification

## Overview

Verify that all adapters properly implement their traits and maintain strict hexagonal architecture boundaries. Ensure no core types leak into adapters and no adapter-specific types appear in core.

## Dependencies

**REQUIRES:**
- ✅ **Task 11** (OpenAI Adapter) - LLMProvider implementation
- ✅ **Task 8** (Qdrant Adapter) - VectorStore implementation
- ✅ **Phase 1** - Core domain types and traits

## Objectives

1. Verify all adapters implement their traits correctly
2. Check for boundary violations (core ↔ adapter)
3. Ensure proper error conversion
4. Validate type conversions

## Implementation Tasks

### 1. Adapter Verification

**Location**: `src/adapters/` (all adapter files)

**Checks**:
- ✅ OpenAI adapter implements `LLMProvider`
- ✅ Qdrant adapter implements `VectorStore`
- ✅ All adapters use `SentinelError` (not external error types)
- ✅ No core types imported in adapters (except through traits)
- ✅ No adapter types in core

### 2. Boundary Checks

**Verification**:
- Run `cargo tree` to verify no circular dependencies
- Check imports in `src/core/` - should have NO:
  - `async_openai`
  - `qdrant_client`
  - `sled`
  - `tokio` (except for tests)
  - `axum`
  - `reqwest`
- Check imports in `src/adapters/` - should have:
  - Core types through `crate::core::*`
  - External crates for their specific implementation

### 3. Error Handling Verification

**Requirements**:
- All adapters convert external errors to `SentinelError`
- No external error types exposed outside adapters
- Error messages are descriptive but don't leak implementation details

### 4. Type Conversion Verification

**Requirements**:
- `CanonicalMessage` ↔ External message types
- `MessageId` ↔ External ID types
- All conversions are explicit and tested

## Testing Requirements

### Integration Tests

**Location**: `tests/adapter_boundaries.rs` (new file)

**Test Cases**:
1. ✅ Verify core has no external dependencies
2. ✅ Verify adapters implement traits correctly
3. ✅ Test error conversion paths
4. ✅ Test type conversion round-trips

## Acceptance Criteria

- [ ] All adapters implement their traits
- [ ] No boundary violations detected
- [ ] `cargo tree` shows clean dependency graph
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Documentation updated with boundary rules

## References

- Architecture Doc: "Hexagonal Architecture" (lines 32-43)
- PRD Section: "Adapter Boundaries" (lines 88-92)

## Next Task

After completing this task, **Phase 4: Integration** is complete. Proceed to **Phase 5: API Layer** (see PRD)

