# Task 14: OpenAPI Schema Generation

## Overview

Generate OpenAPI schema from the API routes to enable frontend type generation and API documentation. Use `utoipa` or similar to automatically generate the schema from route handlers and types.

## Dependencies

**REQUIRES:**
- ✅ **Task 13** - API route handlers implemented
- ✅ **Phase 1** - Core domain types with Serialize/Deserialize

## Objectives

1. Add OpenAPI schema generation using `utoipa`
2. Annotate routes and types with OpenAPI metadata
3. Generate `openapi.yaml` file
4. Ensure schema matches `CanonicalMessage` types
5. Commit schema to repository

## Implementation Tasks

### 1. Add utoipa Dependency

**Location**: `Cargo.toml`

**Requirements**:
- Add `utoipa` with `axum` and `serde` features
- Add `utoipa-swagger-ui` for serving the schema

### 2. Annotate Types with OpenAPI

**Location**: `src/core/types.rs`

**Requirements**:
- Add `#[derive(ToSchema)]` to all API types:
  - `CanonicalMessage`
  - `ChatCompletionRequest`
  - `ChatCompletionResponse`
  - `AgentStatus`
  - `HealthStatus`
  - `ErrorResponse`
  - `TokenUsage`
- Add descriptions and examples where appropriate

### 3. Annotate Routes

**Location**: `src/api/routes.rs`

**Requirements**:
- Add `#[utoipa::path(...)]` annotations to route handlers
- Document request/response types
- Document error responses
- Add operation IDs and tags

### 4. Generate OpenAPI Schema

**Location**: `src/api/mod.rs` or new file

**Requirements**:
- Create OpenAPI spec using `utoipa::OpenApi`
- Include all routes
- Include all types
- Add API metadata (title, version, description)

### 5. Serve OpenAPI Schema

**Location**: `src/api/routes.rs`

**Requirements**:
- Add route to serve OpenAPI JSON: `/openapi.json`
- Add route to serve Swagger UI: `/swagger-ui`
- Use `utoipa-swagger-ui` for UI

### 6. Generate openapi.yaml

**Requirements**:
- Create build script or command to generate `openapi.yaml`
- Commit `openapi.yaml` to repository
- Document generation process

## Testing Requirements

### Manual Testing

**Test Cases**:
1. ✅ `/openapi.json` returns valid OpenAPI 3.0 schema
2. ✅ `/swagger-ui` displays interactive API docs
3. ✅ Schema includes all routes
4. ✅ Schema includes all types
5. ✅ Schema matches actual API behavior

## Acceptance Criteria

- [ ] `utoipa` dependency added
- [ ] All API types annotated with `ToSchema`
- [ ] All routes annotated with `utoipa::path`
- [ ] OpenAPI schema generated and accessible
- [ ] Swagger UI accessible
- [ ] `openapi.yaml` generated and committed
- [ ] Schema validates against OpenAPI 3.0 spec
- [ ] All tests pass

## References

- utoipa documentation: https://docs.rs/utoipa/
- OpenAPI 3.0 Specification
- PRD Section: "API Gateway" (lines 295-327)
- Start Guide: Phase 5 (lines 138-160)

## Next Task

After completing this task, **Phase 5: API Layer** is complete. Proceed to **Phase 6: Integration & Testing** (see PRD)

