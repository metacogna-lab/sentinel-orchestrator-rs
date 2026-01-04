# Task 13: API Route Handlers Implementation

## Overview

Implement the API route handlers to connect HTTP requests to the engine components. Wire up chat completion to use the LLM provider and actor system.

## Dependencies

**REQUIRES:**
- ✅ **Phase 2** - Actor system and supervisor
- ✅ **Phase 4** - LLM provider adapters
- ✅ **Phase 3** - Memory system (for context)

## Objectives

1. Implement chat completion handler with LLM integration
2. Implement agent status handler with supervisor integration
3. Add request validation
4. Connect handlers to engine components
5. Implement streaming responses

## Implementation Tasks

### 1. Chat Completion Handler

**Location**: `src/api/routes.rs`

**Requirements**:
- Accept `ChatCompletionRequest` DTO
- Validate request (non-empty messages, valid roles)
- Convert DTO → `Vec<CanonicalMessage>`
- Call LLM provider (via actor or directly)
- Return `ChatCompletionResponse`
- Support streaming responses

**Code Structure**:
```rust
pub async fn chat_completion(
    State(app_state): State<AppState>,
    auth_info: Option<Extension<AuthInfo>>,
    Json(request): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate request
    // Convert to CanonicalMessage
    // Call LLM provider
    // Return response
}
```

### 2. Agent Status Handler

**Location**: `src/api/routes.rs`

**Requirements**:
- Query supervisor for agent statuses
- Return `Vec<AgentStatus>`
- Include state, last activity, message count

### 3. Request Validation

**Requirements**:
- Validate `ChatCompletionRequest.messages` is non-empty
- Validate message roles are valid
- Validate message content is not empty
- Return appropriate error responses

### 4. App State Structure

**Location**: `src/api/mod.rs` or `src/api/routes.rs`

**Requirements**:
- Hold references to:
  - LLM provider (Arc<dyn LLMProvider>)
  - Supervisor (for agent status)
  - Memory manager (for context)
- Thread-safe access

**Code Structure**:
```rust
pub struct AppState {
    pub llm_provider: Arc<dyn LLMProvider>,
    pub supervisor: Arc<Supervisor>, // If available
    pub memory_manager: Arc<MemoryManager>, // If available
}
```

### 5. Streaming Support

**Requirements**:
- Support `stream: true` in request
- Return Server-Sent Events (SSE) or chunked response
- Stream LLM provider responses

## Testing Requirements

### Unit Tests

**Location**: `src/api/routes.rs`

**Test Cases**:
1. ✅ Chat completion with valid request
2. ✅ Chat completion with invalid request (empty messages)
3. ✅ Chat completion with streaming
4. ✅ Agent status returns correct data
5. ✅ Request validation works

### Integration Tests

**Location**: `tests/api_integration.rs`

**Test Cases**:
- Full HTTP request → LLM → response cycle
- Streaming responses work
- Error handling works

## Acceptance Criteria

- [ ] Chat completion handler works end-to-end
- [ ] Agent status handler works
- [ ] Request validation implemented
- [ ] Streaming responses work
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted

## Error Handling

- Convert `SentinelError` to HTTP status codes
- Return proper error responses (OpenAI-compatible format)
- Log all errors

## References

- PRD Section: "API Gateway" (lines 295-327)
- Architecture Doc: "API Layer" (lines 231-249)
- Start Guide: Phase 5 (lines 138-160)

## Next Task

After completing this task, proceed to **task-14.md**: OpenAPI Schema Generation

