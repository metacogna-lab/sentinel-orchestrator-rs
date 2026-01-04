# Sentinel Orchestrator API Reference

**Base URL**: `http://localhost:3000` (development)  
**API Version**: `v1`  
**Content-Type**: `application/json`

This document provides complete API reference for the Sentinel Orchestrator REST API. All endpoints follow RESTful conventions and return JSON responses.

## Table of Contents

- [Authentication](#authentication)
- [Endpoints](#endpoints)
  - [Chat Completions](#chat-completions)
  - [Health Checks](#health-checks)
- [Request/Response Formats](#requestresponse-formats)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)
- [Examples](#examples)

## Authentication

Currently, the API does not require authentication in development. In production, authentication will be added (API keys or OAuth).

**Future**: API keys via `Authorization: Bearer <token>` header.

## Endpoints

### Chat Completions

#### POST `/v1/chat/completions`

Create a chat completion by sending a conversation to the orchestrator.

**Request Body**:
```json
{
  "messages": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "role": "user",
      "content": "Hello, how are you?",
      "timestamp": "2025-01-20T10:00:00Z",
      "metadata": {}
    }
  ],
  "model": "gpt-4",
  "temperature": 0.7,
  "max_tokens": 1000,
  "stream": false
}
```

**Parameters**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `messages` | `Array<CanonicalMessage>` | Yes | List of messages in the conversation |
| `model` | `String` | No | Model to use (defaults to configured model) |
| `temperature` | `Number` | No | Sampling temperature (0.0-2.0, default: varies) |
| `max_tokens` | `Integer` | No | Maximum tokens to generate |
| `stream` | `Boolean` | No | Stream responses (default: `false`) |

**Response** (200 OK):
```json
{
  "message": {
    "id": "123e4567-e89b-12d3-a456-426614174001",
    "role": "assistant",
    "content": "Hello! I'm doing well, thank you for asking.",
    "timestamp": "2025-01-20T10:00:01Z",
    "metadata": {}
  },
  "model": "gpt-4",
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 12,
    "total_tokens": 22
  }
}
```

**Error Responses**:

| Status Code | Error Code | Description |
|-------------|------------|-------------|
| 400 | `INVALID_REQUEST` | Invalid request format or validation error |
| 401 | `UNAUTHORIZED` | Invalid or missing API key |
| 429 | `RATE_LIMIT_EXCEEDED` | Too many requests |
| 500 | `INTERNAL_ERROR` | Internal server error |
| 503 | `SERVICE_UNAVAILABLE` | Service unavailable (circuit breaker, backpressure) |

**Example Request**:
```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "messages": [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "role": "user",
        "content": "What is Rust?",
        "timestamp": "2025-01-20T10:00:00Z",
        "metadata": {}
      }
    ],
    "model": "gpt-4",
    "temperature": 0.7,
    "max_tokens": 500
  }'
```

**Example Response**:
```json
{
  "message": {
    "id": "123e4567-e89b-12d3-a456-426614174001",
    "role": "assistant",
    "content": "Rust is a systems programming language...",
    "timestamp": "2025-01-20T10:00:02Z",
    "metadata": {}
  },
  "model": "gpt-4",
  "usage": {
    "prompt_tokens": 7,
    "completion_tokens": 150,
    "total_tokens": 157
  }
}
```

### Health Checks

#### GET `/health`

Check the overall health status of the system.

**Response** (200 OK - Healthy):
```json
{
  "status": "healthy",
  "timestamp": "2025-01-20T10:00:00Z"
}
```

**Response** (503 Service Unavailable - Unhealthy):
```json
{
  "status": "unhealthy",
  "timestamp": "2025-01-20T10:00:00Z"
}
```

**Use Cases**:
- Load balancer health checks
- Monitoring systems
- Deployment verification

#### GET `/health/ready`

Check if the system is ready to accept requests (all components initialized).

**Response** (200 OK - Ready):
```json
{
  "status": "ready",
  "timestamp": "2025-01-20T10:00:00Z"
}
```

**Response** (503 Service Unavailable - Not Ready):
```json
{
  "status": "unhealthy",
  "timestamp": "2025-01-20T10:00:00Z"
}
```

**Checks**:
- Database connectivity (PostgreSQL)
- Vector store connectivity (Weaviate)
- All adapters initialized
- Memory manager ready

**Use Cases**:
- Kubernetes readiness probes
- Deployment readiness checks
- System startup verification

#### GET `/health/live`

Basic liveness check (always returns 200 if the service is running).

**Response** (200 OK):
```json
{
  "status": "alive",
  "timestamp": "2025-01-20T10:00:00Z"
}
```

**Use Cases**:
- Kubernetes liveness probes
- Basic service availability checks
- Process health monitoring

**Note**: This endpoint does not check dependencies, only that the HTTP server is responding.

## Request/Response Formats

### CanonicalMessage

All messages use the `CanonicalMessage` format:

```typescript
interface CanonicalMessage {
  id: string;              // UUID v4
  role: "user" | "assistant" | "system";
  content: string;         // Message content (non-empty)
  timestamp: string;       // ISO 8601 datetime (UTC)
  metadata?: Record<string, string>;  // Optional key-value pairs
}
```

**Validation Rules**:
- `id`: Must be a valid UUID v4
- `role`: Must be one of: `user`, `assistant`, `system`
- `content`: Must not be empty (after trimming)
- `timestamp`: Must be valid ISO 8601 format, not more than 1 hour in the future, not more than 100 years in the past
- `metadata`: Optional, key-value pairs (strings only)

### ChatCompletionRequest

```typescript
interface ChatCompletionRequest {
  messages: CanonicalMessage[];
  model?: string;
  temperature?: number;      // 0.0 - 2.0
  max_tokens?: number;       // Positive integer
  stream?: boolean;          // Default: false
}
```

### ChatCompletionResponse

```typescript
interface ChatCompletionResponse {
  message: CanonicalMessage;
  model: string;
  usage?: TokenUsage;
}

interface TokenUsage {
  prompt_tokens: number;
  completion_tokens: number;
  total_tokens: number;
}
```

### ErrorResponse

```typescript
interface ErrorResponse {
  code: string;
  message: string;
  details?: Record<string, string>;
}
```

**Error Codes**:

| Code | Description |
|------|-------------|
| `INVALID_REQUEST` | Request validation failed |
| `UNAUTHORIZED` | Authentication required |
| `RATE_LIMIT_EXCEEDED` | Too many requests |
| `INTERNAL_ERROR` | Internal server error |
| `SERVICE_UNAVAILABLE` | Service unavailable (circuit breaker, backpressure) |
| `INVALID_STATE_TRANSITION` | Agent state transition error |
| `INVALID_MESSAGE` | Message validation failed |
| `DOMAIN_VIOLATION` | Domain rule violation |

## Error Handling

All errors follow a consistent format:

```json
{
  "code": "INVALID_REQUEST",
  "message": "Invalid request format",
  "details": {
    "field": "messages",
    "reason": "Cannot be empty"
  }
}
```

### HTTP Status Codes

| Status Code | Meaning | When Used |
|-------------|---------|-----------|
| 200 | OK | Successful request |
| 400 | Bad Request | Invalid request format or validation error |
| 401 | Unauthorized | Authentication required or invalid |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Unexpected server error |
| 503 | Service Unavailable | Circuit breaker open, backpressure, or system overload |

### Error Response Format

All error responses include:
- `code`: Machine-readable error code
- `message`: Human-readable error message
- `details` (optional): Additional error context

**Example Error Response**:
```json
{
  "code": "INVALID_MESSAGE",
  "message": "Message content cannot be empty",
  "details": {
    "field": "content",
    "validation_rule": "content must not be empty after trimming"
  }
}
```

## Rate Limiting

**Current Status**: Rate limiting is planned for Phase 6 (Resilience & Production Hardening).

**Planned Limits**:
- Global: 100 requests per minute
- Per-IP: 10 requests per minute

**Response Headers** (when implemented):
- `X-RateLimit-Limit`: Request limit per window
- `X-RateLimit-Remaining`: Remaining requests in window
- `X-RateLimit-Reset`: Unix timestamp when limit resets
- `Retry-After`: Seconds to wait before retrying (on 429)

**429 Response**:
```json
{
  "code": "RATE_LIMIT_EXCEEDED",
  "message": "Too many requests. Please try again later.",
  "details": {
    "retry_after": "60",
    "limit": "100",
    "window": "1 minute"
  }
}
```

## Examples

### Complete Conversation

```bash
# Initial message
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "messages": [
      {
        "id": "msg-1",
        "role": "user",
        "content": "Hello!",
        "timestamp": "2025-01-20T10:00:00Z",
        "metadata": {}
      }
    ]
  }'

# Follow-up message
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "messages": [
      {
        "id": "msg-1",
        "role": "user",
        "content": "Hello!",
        "timestamp": "2025-01-20T10:00:00Z",
        "metadata": {}
      },
      {
        "id": "msg-2",
        "role": "assistant",
        "content": "Hi there! How can I help you?",
        "timestamp": "2025-01-20T10:00:01Z",
        "metadata": {}
      },
      {
        "id": "msg-3",
        "role": "user",
        "content": "What can you do?",
        "timestamp": "2025-01-20T10:00:05Z",
        "metadata": {}
      }
    ]
  }'
```

### With System Message

```json
{
  "messages": [
    {
      "id": "sys-1",
      "role": "system",
      "content": "You are a helpful assistant specializing in Rust programming.",
      "timestamp": "2025-01-20T10:00:00Z",
      "metadata": {}
    },
    {
      "id": "msg-1",
      "role": "user",
      "content": "Explain ownership in Rust",
      "timestamp": "2025-01-20T10:00:01Z",
      "metadata": {}
    }
  ]
}
```

### Health Check Monitoring

```bash
# Basic health check
curl http://localhost:3000/health

# Readiness check
curl http://localhost:3000/health/ready

# Liveness check
curl http://localhost:3000/health/live
```

## OpenAPI Specification

For the complete OpenAPI 3.0 specification, see [`openapi.yaml`](../openapi.yaml).

The OpenAPI spec includes:
- Complete schema definitions
- Request/response examples
- Error response formats
- Authentication requirements (when implemented)

## SDK and Client Libraries

**Rust Client** (planned):
```rust
use sentinel_client::Client;

let client = Client::new("http://localhost:3000")?;
let response = client.chat_completions(request).await?;
```

**TypeScript/JavaScript Client** (planned):
```typescript
import { SentinelClient } from '@sentinel/client';

const client = new SentinelClient('http://localhost:3000');
const response = await client.chatCompletions(request);
```

**Python Client** (planned):
```python
from sentinel_client import SentinelClient

client = SentinelClient("http://localhost:3000")
response = client.chat_completions(request)
```

## Versioning

The API uses URL versioning: `/v1/...`

**Current Version**: `v1`

**Future Versions**: `/v2/...`, `/v3/...`, etc.

Breaking changes will result in a new version number. Non-breaking additions may be made to the current version.

## Support

For API issues or questions:
- Check the [Usage Guide](./usage.md) for examples
- Review the [Type System](./types.md) for data structure details
- Consult the [OpenAPI Specification](../openapi.yaml) for complete schema

---

**Last Updated**: 2025-01-20  
**API Version**: 1.0.0

