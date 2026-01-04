# Sentinel Orchestrator - Agent Instructions

## Build/Test Commands
- **Build**: `cargo build`
- **Test all**: `cargo test`
- **Test single**: `cargo test test_name` (replace test_name with actual test function name)
- **Run**: `cargo run`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`

## Code Style Guidelines

### Architecture
- Follow Hexagonal Architecture: `src/core/` contains pure domain logic with NO external I/O dependencies
- Traits in `core/traits.rs` define interfaces; adapters in `adapters/` implement them
- Use `async-trait` for async trait methods

### Error Handling
- Use `thiserror` in `src/core` for domain errors
- Use `anyhow` in binaries/main.rs for application errors
- Always use `Result<T, E>` - never `unwrap()` or `expect()` in production

### Async Patterns
- Use bounded channels (`mpsc::channel(N)`) for backpressure
- Use `tokio::spawn` for concurrent tasks, `tokio::select!` for cancellation/timeouts
- Prefer `async fn` over manual `Future` implementations

### Naming & Types
- Use `Arc<T>` for shared ownership (thread-safe)
- Prefer `&str` over `String` in function parameters
- Use `Vec::with_capacity()` when size is known

### Testing
- Use `mockall` for trait mocking, `#[tokio::test]` for async tests
- Integration tests in `tests/` directory
- Test error paths, not just happy paths

### Cursor Rules
Follow all rules in `.cursor/rules/*.mdc` files for detailed Rust-specific guidelines.