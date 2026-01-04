# Sentinel Orchestrator - Canonical Documentation Guide

**Version**: 1.0  
**Last Updated**: 2025-01-20

This is the canonical documentation guide for the Sentinel Rust Orchestrator. It provides comprehensive coverage of the API, types, backend architecture, and usage patterns.

## Documentation Structure

### [API Reference](./api.md)
Complete API documentation covering:
- REST endpoints and request/response formats
- Authentication and error handling
- Rate limiting and quotas
- WebSocket/streaming interfaces (if applicable)
- OpenAPI schema reference

### [Type System](./types.md)
Comprehensive type documentation including:
- Domain types (`CanonicalMessage`, `AgentState`, etc.)
- ID types (`MessageId`, `AgentId`)
- Request/Response DTOs
- Error types and handling
- Type relationships and constraints

### [Usage Guide](./usage.md)
Practical guides and examples:
- Quick start guide
- Common usage patterns
- Code examples (Rust and API)
- Best practices
- Troubleshooting
- Migration guides

### [Backend Architecture](./backend.md)
Backend implementation documentation:
- Hexagonal Architecture principles
- Module organization
- Actor system and state machines
- Memory management
- Adapter patterns
- Testing strategies

### [Architecture Overview](./architecture.md)
High-level system design:
- Architectural principles
- Component relationships
- Data flow diagrams
- Design decisions
- Research references

### [Product Requirements](./prd.md)
Complete product requirements document:
- Phase-by-phase implementation plans
- Acceptance criteria
- Risk assessment
- Timeline estimates

## Quick Navigation

### For API Consumers
- Start with [API Reference](./api.md) for endpoint documentation
- Review [Type System](./types.md) for data structures
- Follow [Usage Guide](./usage.md) for examples

### For Backend Developers
- Read [Backend Architecture](./backend.md) for implementation details
- Review [Architecture Overview](./architecture.md) for design principles
- Consult [Type System](./types.md) for domain model

### For System Architects
- Review [Architecture Overview](./architecture.md)
- Consult [Product Requirements](./prd.md) for system scope
- Reference [Backend Architecture](./backend.md) for implementation details

## Key Concepts

### Canonical Message Model
All communication uses `CanonicalMessage` - a pure Rust type with no external dependencies. This ensures the domain layer remains independent of infrastructure choices.

### Hexagonal Architecture
Strict separation between:
- **Core Domain** (`src/core/`): Pure types, traits, and domain logic
- **Adapters** (`src/adapters/`): Infrastructure implementations
- **Engine** (`src/engine/`): Orchestration and state management
- **API** (`src/api/`): HTTP gateway layer

### Actor Model
Agents communicate via message-passing channels (`tokio::sync::mpsc`), avoiding shared mutable state and ensuring thread safety through Rust's ownership system.

### Three-Tier Memory
- **Short-Term**: In-memory conversation history (fast access)
- **Medium-Term**: Sled database (persistent, summarized)
- **Long-Term**: Weaviate vector store (semantic search)

## Documentation Standards

All documentation follows these principles:
1. **Accuracy**: Documentation matches implementation
2. **Completeness**: All public APIs are documented
3. **Examples**: Code examples for common patterns
4. **Clarity**: Clear explanations with context
5. **Maintenance**: Documentation updated with code changes

## Contributing to Documentation

When contributing code:
1. Add Rust doc comments (`///`) for all public items
2. Update relevant markdown documentation
3. Include code examples in doc comments
4. Update this guide if adding new documentation sections

## Related Resources

- [Rust API Documentation](../target/doc/sentinel/index.html) - Generated Rust docs
- [OpenAPI Specification](../openapi.yaml) - API contract definition
- [Research Comparison](./research/rust_orchestrators_comparison.md) - Framework analysis
- [Agent Roles](../AGENTS.md) - Agent system documentation

---

**Note**: This documentation is actively maintained. For the latest updates, check the repository's `docs/` directory.

