# Sentinel Orchestrator Usage Guide

Practical guide for using the Sentinel Orchestrator API and integrating with the system.

## Table of Contents

- [Quick Start](#quick-start)
- [Common Patterns](#common-patterns)
- [Rust Examples](#rust-examples)
- [API Examples](#api-examples)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [Integration Examples](#integration-examples)

## Quick Start

### Prerequisites

1. **Install Rust** (for backend development):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Set up environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your API keys and configuration
   ```

3. **Start services** (using Docker Compose):
   ```bash
   docker-compose up -d
   ```

4. **Run the backend**:
   ```bash
   cargo run
   ```

### First API Call

```bash
curl -X POST http://localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "messages": [
      {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "role": "user",
        "content": "Hello!",
        "timestamp": "2025-01-20T10:00:00Z",
        "metadata": {}
      }
    ]
  }'
```

## Common Patterns

### Basic Conversation

Send a message and receive a response:

```json
{
  "messages": [
    {
      "id": "msg-1",
      "role": "user",
      "content": "What is Rust?",
      "timestamp": "2025-01-20T10:00:00Z",
      "metadata": {}
    }
  ]
}
```

### Multi-Turn Conversation

Include conversation history:

```json
{
  "messages": [
    {
      "id": "msg-1",
      "role": "user",
      "content": "What is ownership?",
      "timestamp": "2025-01-20T10:00:00Z",
      "metadata": {}
    },
    {
      "id": "msg-2",
      "role": "assistant",
      "content": "Ownership is Rust's memory management system...",
      "timestamp": "2025-01-20T10:00:01Z",
      "metadata": {}
    },
    {
      "id": "msg-3",
      "role": "user",
      "content": "Give me an example",
      "timestamp": "2025-01-20T10:00:05Z",
      "metadata": {}
    }
  ]
}
```

### With System Context

Set system instructions:

```json
{
  "messages": [
    {
      "id": "sys-1",
      "role": "system",
      "content": "You are a helpful Rust programming assistant.",
      "timestamp": "2025-01-20T10:00:00Z",
      "metadata": {}
    },
    {
      "id": "msg-1",
      "role": "user",
      "content": "Explain borrowing",
      "timestamp": "2025-01-20T10:00:01Z",
      "metadata": {}
    }
  ]
}
```

### With Metadata

Include metadata for tracking:

```json
{
  "messages": [
    {
      "id": "msg-1",
      "role": "user",
      "content": "Hello",
      "timestamp": "2025-01-20T10:00:00Z",
      "metadata": {
        "source": "web",
        "user_id": "user-123",
        "session_id": "session-456"
      }
    }
  ]
}
```

## Rust Examples

### Creating Messages

```rust
use sentinel::core::types::{CanonicalMessage, Role};
use chrono::Utc;

// Create a new message
let msg = CanonicalMessage::new(
    Role::User,
    "Hello, world!".to_string()
);

// Create with explicit timestamp
let msg_with_time = CanonicalMessage::with_timestamp(
    Role::User,
    "Hello".to_string(),
    Utc::now()
);

// Create with metadata
use std::collections::HashMap;
let mut metadata = HashMap::new();
metadata.insert("source".to_string(), "api".to_string());
let msg_with_meta = CanonicalMessage::with_metadata(
    Role::User,
    "Hello".to_string(),
    metadata
);

// Validate a message
match msg.validate_self() {
    Ok(()) => println!("Message is valid"),
    Err(e) => eprintln!("Validation error: {}", e),
}
```

### Working with IDs

```rust
use sentinel::core::types::{MessageId, AgentId};
use uuid::Uuid;

// Generate new IDs
let message_id = MessageId::new();
let agent_id = AgentId::new();

// Convert to/from UUID
let uuid: Uuid = message_id.into();
let message_id_from_uuid = MessageId::from(uuid);

// Display
println!("Message ID: {}", message_id);
```

### State Transitions

```rust
use sentinel::core::types::AgentState;
use sentinel::core::error::SentinelError;

// Validate state transition
match AgentState::validate_transition(
    AgentState::Idle,
    AgentState::Thinking
) {
    Ok(()) => println!("Valid transition"),
    Err(SentinelError::InvalidStateTransition { from, to }) => {
        eprintln!("Invalid transition: {:?} -> {:?}", from, to);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Error Handling

```rust
use sentinel::core::error::SentinelError;

fn handle_error(error: SentinelError) {
    match error {
        SentinelError::InvalidStateTransition { from, to } => {
            eprintln!("Invalid state transition: {:?} -> {:?}", from, to);
        }
        SentinelError::InvalidMessage { reason } => {
            eprintln!("Invalid message: {}", reason);
        }
        SentinelError::DomainViolation { rule } => {
            eprintln!("Domain violation: {}", rule);
        }
    }
}
```

## API Examples

### Python Example

```python
import requests
import uuid
from datetime import datetime, timezone

def send_message(content: str):
    url = "http://localhost:3000/v1/chat/completions"
    
    payload = {
        "messages": [
            {
                "id": str(uuid.uuid4()),
                "role": "user",
                "content": content,
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "metadata": {}
            }
        ]
    }
    
    response = requests.post(url, json=payload)
    response.raise_for_status()
    return response.json()

# Usage
result = send_message("Hello!")
print(result["message"]["content"])
```

### JavaScript/TypeScript Example

```typescript
async function sendMessage(content: string) {
  const url = "http://localhost:3000/v1/chat/completions";
  
  const payload = {
    messages: [
      {
        id: crypto.randomUUID(),
        role: "user",
        content: content,
        timestamp: new Date().toISOString(),
        metadata: {}
      }
    ]
  };
  
  const response = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload)
  });
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  return await response.json();
}

// Usage
const result = await sendMessage("Hello!");
console.log(result.message.content);
```

### Go Example

```go
package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "net/http"
    "time"
    "github.com/google/uuid"
)

type Message struct {
    ID        string            `json:"id"`
    Role      string            `json:"role"`
    Content   string            `json:"content"`
    Timestamp string            `json:"timestamp"`
    Metadata  map[string]string `json:"metadata"`
}

type Request struct {
    Messages []Message `json:"messages"`
}

func sendMessage(content string) error {
    url := "http://localhost:3000/v1/chat/completions"
    
    msg := Message{
        ID:        uuid.New().String(),
        Role:      "user",
        Content:   content,
        Timestamp: time.Now().UTC().Format(time.RFC3339),
        Metadata:  make(map[string]string),
    }
    
    req := Request{
        Messages: []Message{msg},
    }
    
    jsonData, err := json.Marshal(req)
    if err != nil {
        return err
    }
    
    resp, err := http.Post(url, "application/json", bytes.NewBuffer(jsonData))
    if err != nil {
        return err
    }
    defer resp.Body.Close()
    
    var result map[string]interface{}
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return err
    }
    
    fmt.Println(result)
    return nil
}
```

## Best Practices

### Message ID Generation

Always generate unique IDs for each message:

```rust
// Good: Generate new ID
let msg = CanonicalMessage::new(Role::User, content);

// Bad: Reusing IDs (can cause confusion)
// Use MessageId::new() for each message
```

### Timestamp Handling

Use UTC timestamps consistently:

```rust
use chrono::Utc;

// Good: Use UTC
let timestamp = Utc::now();

// Bad: Using local time
// let timestamp = Local::now();  // Don't do this
```

### Error Handling

Always handle errors appropriately:

```rust
// Good: Explicit error handling
match msg.validate_self() {
    Ok(()) => { /* proceed */ }
    Err(e) => {
        eprintln!("Validation failed: {}", e);
        // Handle error appropriately
    }
}

// Bad: Ignoring errors
// msg.validate_self();  // Don't ignore errors
```

### Conversation History Management

Keep conversation history manageable:

```rust
// Good: Limit conversation history
let recent_messages: Vec<CanonicalMessage> = messages
    .iter()
    .rev()
    .take(10)  // Keep last 10 messages
    .cloned()
    .collect();

// The system handles memory consolidation automatically,
// but you can optimize by sending only recent messages
```

### Metadata Usage

Use metadata for tracking and context:

```rust
// Good: Include useful metadata
let mut metadata = HashMap::new();
metadata.insert("source".to_string(), "web".to_string());
metadata.insert("user_id".to_string(), user_id.clone());
metadata.insert("session_id".to_string(), session_id.clone());

let msg = CanonicalMessage::with_metadata(
    Role::User,
    content,
    metadata
);
```

### Health Checks

Monitor system health:

```bash
# Check overall health
curl http://localhost:3000/health

# Check readiness (all components initialized)
curl http://localhost:3000/health/ready

# Check liveness (basic availability)
curl http://localhost:3000/health/live
```

## Troubleshooting

### Common Issues

#### Invalid Message Error

**Problem**: `INVALID_MESSAGE` error returned

**Possible Causes**:
- Empty message content (after trimming)
- Invalid timestamp (too far in future/past)
- Missing required fields

**Solution**:
```rust
// Validate message before sending
match msg.validate_self() {
    Ok(()) => { /* send message */ }
    Err(e) => {
        // Fix the issue before sending
        eprintln!("Message validation failed: {}", e);
    }
}
```

#### State Transition Error

**Problem**: `INVALID_STATE_TRANSITION` error

**Possible Causes**:
- Attempting invalid state transition
- State machine logic error

**Solution**:
```rust
// Validate transition before applying
match AgentState::validate_transition(current_state, new_state) {
    Ok(()) => { /* apply transition */ }
    Err(e) => {
        eprintln!("Invalid transition: {}", e);
        // Handle appropriately
    }
}
```

#### Service Unavailable (503)

**Problem**: `SERVICE_UNAVAILABLE` error

**Possible Causes**:
- Circuit breaker open (too many failures)
- Backpressure (system overloaded)
- Database connection issues

**Solution**:
1. Check system health: `curl http://localhost:3000/health`
2. Wait and retry (with exponential backoff)
3. Check logs for underlying issues
4. Reduce request rate if causing backpressure

#### Connection Errors

**Problem**: Cannot connect to API

**Possible Causes**:
- Service not running
- Wrong port/URL
- Network issues

**Solution**:
1. Verify service is running: `curl http://localhost:3000/health/live`
2. Check port configuration
3. Verify network connectivity
4. Check firewall settings

## Integration Examples

### Web Application Integration

```typescript
// React component example
import { useState } from 'react';

function ChatInterface() {
  const [messages, setMessages] = useState<CanonicalMessage[]>([]);
  const [input, setInput] = useState('');
  
  async function sendMessage() {
    const newMessage: CanonicalMessage = {
      id: crypto.randomUUID(),
      role: 'user',
      content: input,
      timestamp: new Date().toISOString(),
      metadata: {}
    };
    
    const response = await fetch('http://localhost:3000/v1/chat/completions', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        messages: [...messages, newMessage]
      })
    });
    
    const data = await response.json();
    setMessages([...messages, newMessage, data.message]);
    setInput('');
  }
  
  return (
    <div>
      {/* Chat UI */}
    </div>
  );
}
```

### CLI Tool Integration

```rust
// CLI example using clap
use clap::Parser;
use sentinel::core::types::{CanonicalMessage, Role};

#[derive(Parser)]
struct Args {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let msg = CanonicalMessage::new(Role::User, args.message);
    msg.validate_self()?;
    
    // Send to API
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/v1/chat/completions")
        .json(&serde_json::json!({
            "messages": [msg]
        }))
        .send()
        .await?;
    
    let result: serde_json::Value = response.json().await?;
    println!("{}", result["message"]["content"]);
    
    Ok(())
}
```

---

**See Also**:
- [API Reference](./api.md) - Complete API documentation
- [Type System](./types.md) - Type definitions and validation
- [Backend Architecture](./backend.md) - Implementation details

