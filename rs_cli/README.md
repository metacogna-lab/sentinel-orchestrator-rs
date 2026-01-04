# Sentinel Orchestrator CLI

A beautiful, interactive command-line interface for managing and interacting with the Sentinel Orchestrator backend. Built with Rust using best-in-class TUI libraries, featuring full keyboard navigation, real-time streaming, and multiple operational modes.

## Features

### 🎨 Beautiful TUI Interface
- Modern, color-coded interface using `ratatui` (the most respected Rust TUI library)
- Intuitive navigation with keyboard shortcuts
- Real-time updates and streaming responses
- Error handling with user-friendly messages

### 🔄 Multiple Operational Modes

#### 1. **Chat Mode**
- Interactive chat interface with the Sentinel backend
- Real-time streaming responses from LLM completions
- Conversation history display
- Color-coded messages by role (User, Assistant, System)

#### 2. **Investigation Mode**
- Query and investigate system state
- Search through memory and logs
- Display investigation results in organized format

#### 3. **Debugging Mode**
- View system debug logs
- Color-coded log levels (ERROR, WARN, INFO)
- Real-time log updates
- Scrollable log history (last 100 entries)

#### 4. **System Status Mode**
- Health check monitoring
- Readiness and liveness status
- System endpoint information
- Real-time status updates

### ⌨️ Keyboard Navigation

- **Tab** - Cycle through modes (Main Menu → Chat → Investigation → Debugging → System Status)
- **↑/↓** - Navigate menu items
- **Enter** - Select menu item or send message
- **Esc** - Go back to previous mode or exit from main menu
- **q** - Quit application
- **Backspace** - Delete character in input fields

## Architecture

The CLI follows the same architectural principles as the backend:

### Module Structure

```
rs_cli/
├── src/
│   ├── main.rs           # Application entry point and event loop
│   ├── types.rs          # Type definitions (compatible with backend)
│   ├── api/
│   │   ├── client.rs     # HTTP client for backend communication
│   │   └── mod.rs
│   ├── app/
│   │   ├── state.rs      # Application state management
│   │   ├── handlers.rs   # Event handlers for different modes
│   │   └── mod.rs
│   ├── ui/
│   │   ├── components.rs # TUI rendering components
│   │   └── mod.rs
│   └── modes/
│       └── mod.rs        # Mode definitions
├── Cargo.toml
└── README.md
```

### Key Design Decisions

1. **Type Compatibility**: The CLI uses types that mirror `src/core/types.rs` to ensure perfect compatibility with the backend API.

2. **Async Architecture**: Built on `tokio` for async/await support, matching the backend's async patterns.

3. **Separation of Concerns**:
   - `api/` - HTTP client layer (communicates with backend)
   - `app/` - Application state and business logic
   - `ui/` - Presentation layer (TUI rendering)
   - `modes/` - Mode definitions and state

4. **Error Handling**: Uses `anyhow` for error context, providing user-friendly error messages.

5. **Streaming Support**: Implements real-time streaming for chat completions, displaying responses as they arrive.

## Dependencies

### Core Libraries
- **ratatui** (0.27) - Terminal UI framework (most respected Rust TUI library)
- **crossterm** (0.28) - Cross-platform terminal manipulation
- **clap** (4.5) - Command-line argument parsing
- **reqwest** (0.12) - HTTP client with streaming support
- **tokio** (1.x) - Async runtime

### Supporting Libraries
- **serde** / **serde_json** - Serialization
- **colored** / **owo-colors** - Terminal colors
- **anyhow** / **thiserror** - Error handling
- **chrono** - Time handling
- **uuid** - Message ID generation
- **futures** - Async stream utilities

## Usage

### Building

```bash
cd rs_cli
cargo build --release
```

### Running

```bash
# Default (connects to http://localhost:3000)
cargo run --release

# Custom backend URL
cargo run --release -- --url http://localhost:8080

# With API key authentication
cargo run --release -- --url http://localhost:3000 --api-key sk-your-api-key-here

# Or use environment variable
export SENTINEL_API_KEY=sk-your-api-key-here
cargo run --release
```

### Command-Line Options

```
sentinel-cli [OPTIONS]

Options:
  -u, --url <URL>         Backend API base URL [default: http://localhost:3000]
  -k, --api-key <KEY>     API key for authentication (or set SENTINEL_API_KEY env var)
  -h, --help              Print help
```

### Authentication

The CLI supports API key authentication as implemented in the backend (see `tasks/bridge_auth.md`):

- **Command-line**: Use `--api-key` or `-k` flag
- **Environment Variable**: Set `SENTINEL_API_KEY` environment variable
- **Header Format**: Uses `Authorization: Bearer <key>` (OpenAI-compatible)

The API key is automatically included in all requests to authenticated endpoints. Health check endpoints (`/health`, `/health/live`) remain accessible without authentication.

## Integration with Backend

The CLI communicates with the Sentinel Orchestrator backend through its REST API:

### Endpoints Used

- `GET /health` - Health check (public, no auth required)
- `GET /health/ready` - Readiness check (public, no auth required)
- `GET /health/live` - Liveness check (public, no auth required)
- `POST /v1/chat/completions` - Chat completions (requires Write-level API key, with streaming support)

All authenticated endpoints automatically include the `Authorization: Bearer <key>` header when an API key is provided.

### Type Compatibility

All types in `src/types.rs` are designed to be compatible with `src/core/types.rs`:
- `CanonicalMessage`
- `AgentState`
- `HealthStatus`
- `ChatCompletionRequest` / `ChatCompletionResponse`
- `AgentStatus`

## Development

### Adding New Modes

1. Add mode variant to `src/modes/mod.rs`
2. Add UI component in `src/ui/components.rs`
3. Add handler in `src/app/handlers.rs` (if needed)
4. Update `src/main.rs` to handle the new mode

### Adding New Features

- Follow the existing architectural patterns
- Maintain type compatibility with backend
- Use `ratatui` for all UI rendering
- Handle errors gracefully with user-friendly messages

## Best Practices

1. **No Backend Changes**: The CLI is designed to work with the existing backend API without requiring any backend modifications.

2. **Type Safety**: All types mirror the backend to ensure compile-time safety.

3. **Error Handling**: Always provide context with errors using `anyhow::Context`.

4. **UI/UX**: 
   - Use consistent color schemes
   - Provide clear navigation hints
   - Handle edge cases gracefully

5. **Performance**: 
   - Use async/await for I/O operations
   - Stream responses when possible
   - Avoid blocking the UI thread

## Future Enhancements

- [ ] Real-time agent status monitoring
- [ ] Memory tier visualization
- [ ] Token usage tracking and display
- [ ] Configuration file support
- [ ] Command history and autocomplete
- [ ] Multi-agent monitoring dashboard
- [ ] Export conversation history
- [ ] Custom themes and color schemes

## References

- [Backend Architecture](../docs/architecture.md)
- [API Documentation](../docs/api.md)
- [Product Requirements](../tasks/prd.md)

## License

Part of the Sentinel Orchestrator project.

