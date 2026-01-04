# Sentinel Orchestrator - CLI Agent Instructions

This is the Sentinel Orchestrator, a production-grade agentic system built with Rust following Hexagonal Architecture.

## Architecture

- **Core**: Pure domain logic with no external I/O dependencies
- **Adapters**: Infrastructure implementations (OpenAI, Qdrant, Sled)
- **Engine**: Orchestrator with event loops and channels
- **API**: Gateway layer using Axum
- **Memory**: Hierarchy management (Short/Medium/Long term)
- **Telemetry**: Observability and tracing

## Key Patterns

- **Resilience**: Backpressure handling, circuit breakers, budget kill switches
- **Correctness**: Deterministic replay, strict JSON validation
- **Agentic**: Interrupt persistence, zombie detection

## Development

Follow the Rust-specific rules in `.cursor/rules/*.mdc` for coding standards.

