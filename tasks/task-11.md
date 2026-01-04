# Task 11: OpenAI Adapter Implementation

## Overview

Implement the OpenAI adapter to complete the LLM provider integration. This adapter wraps the async-openai client and implements the `LLMProvider` trait, maintaining strict hexagonal architecture boundaries.

## Dependencies

**REQUIRES:**
- ✅ **Phase 1** - `LLMProvider` trait defined in `src/core/traits.rs`
- ✅ **Phase 3** - Memory system for context retrieval

## Objectives

1. Implement OpenAI adapter with `LLMProvider` trait
2. Support both completion and streaming modes
3. Maintain strict adapter boundaries (no core types in adapters)
4. Handle API key management securely
5. Convert OpenAI types to `CanonicalMessage`

## Implementation Tasks

### 1. OpenAI Adapter Structure

**Location**: `src/adapters/openai.rs` (extend existing)

**Requirements**:
- Implement `LLMProvider` trait
- Wrap `async_openai::Client`
- Convert OpenAI message types to `CanonicalMessage`
- Handle API errors and convert to `SentinelError`

**Code Structure**:
```rust
pub struct OpenAIProvider {
    client: async_openai::Client,
    model: String,
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn complete(
        &self,
        messages: Vec<CanonicalMessage>,
    ) -> Result<CanonicalMessage, SentinelError> {
        // Convert CanonicalMessage to OpenAI ChatMessageRequest
        // Call OpenAI API
        // Convert response to CanonicalMessage
    }
    
    async fn stream(
        &self,
        messages: Vec<CanonicalMessage>,
    ) -> Result<Box<dyn Stream<...>>, SentinelError> {
        // Convert and stream
    }
}
```

### 2. Message Type Conversion

**Requirements**:
- Convert `CanonicalMessage` → OpenAI `ChatMessageRequest`
- Convert OpenAI response → `CanonicalMessage`
- Handle role mapping (User, Assistant, System)
- Preserve message content

**Role Mapping**:
- `Role::User` → `async_openai::types::Role::User`
- `Role::Assistant` → `async_openai::types::Role::Assistant`
- `Role::System` → `async_openai::types::Role::System`

### 3. Error Handling

**Requirements**:
- Convert OpenAI API errors to `SentinelError`
- Handle rate limits gracefully
- Handle authentication errors
- Log errors with context

**Error Mapping**:
- API errors → `SentinelError::DomainViolation`
- Invalid requests → `SentinelError::InvalidMessage`
- Network errors → `SentinelError::DomainViolation`

### 4. Configuration

**Environment Variables**:
- `OPENAI_API_KEY` - OpenAI API key (required)
- `OPENAI_MODEL` - Model to use (default: `gpt-4`)
- `OPENAI_BASE_URL` - Optional custom base URL

### 5. Streaming Support

**Requirements**:
- Implement streaming response handling
- Convert stream chunks to `Result<String, SentinelError>`
- Handle stream errors gracefully
- Support cancellation

## Testing Requirements

### Unit Tests

**Location**: `src/adapters/openai.rs`

**Test Cases**:
1. ✅ OpenAI provider initializes correctly
2. ✅ Message conversion works (CanonicalMessage → OpenAI)
3. ✅ Response conversion works (OpenAI → CanonicalMessage)
4. ✅ Error handling works
5. ✅ Streaming works (with mocked client)

**Test Pattern**:
```rust
#[tokio::test]
async fn test_openai_complete() {
    let provider = OpenAIProvider::new("test-key", "gpt-4").await?;
    
    let messages = vec![
        CanonicalMessage::new(Role::User, "Hello".to_string()),
    ];
    
    let response = provider.complete(messages).await?;
    assert_eq!(response.role, Role::Assistant);
}
```

### Integration Tests

**Location**: `tests/openai_integration.rs` (optional)

**Test Cases**:
- Real OpenAI API calls (requires API key)
- Streaming responses
- Error scenarios

## Acceptance Criteria

- [ ] OpenAI adapter implements `LLMProvider` trait
- [ ] Message conversion works correctly
- [ ] Both completion and streaming work
- [ ] Error handling is comprehensive
- [ ] All tests pass: `cargo test adapters::openai`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] API key handled securely (no logging)
- [ ] Strict adapter boundaries maintained

## Error Handling

- Use `SentinelError` for all errors
- Never expose OpenAI-specific error types
- Log errors with context but not sensitive data
- Handle rate limits with appropriate errors

## Security Considerations

- Never log API keys
- Use `secrecy::Secret<String>` for API key storage
- Validate API key format (if possible)
- Handle authentication errors gracefully

## References

- PRD Section: "LLM Provider Trait" (lines 190-197)
- Architecture Doc: "Adapter Boundaries" (lines 32-43)
- async-openai crate documentation

## Next Task

After completing this task, proceed to **task-12.md**: Adapter Boundary Verification

