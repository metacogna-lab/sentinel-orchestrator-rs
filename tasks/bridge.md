# Task Bridge - State Management

Store Current and next state here.

## Current State
- Conducted Codex code review focused on Phases 1 & 2 per `docs/architecture.md`, `tasks/prd.md`, and `.cursor/rules/*`.
- Updated `AGENTS.md` to formalize the reviewer protocol and enforced logging of findings in `docs/code-review/`.
- Authored review artifacts:
  - `docs/code-review/phase1-phase2.mdx` (core + actor gaps, secret handling).
  - `docs/code-review/security.mdx` (API key storage, error messages, env validation).
  - `docs/code-review/api.mdx` (placeholder handlers, duplicate middleware, DTO validation).
  - `docs/code-review/frontend.mdx` (SPA scaffolding status, Bun workflow).
- Frontend documentation now mandates Bun-only workflows (`frontend/README.md`) and the PRD + bridge files remain the source of truth.
- No Rust engine components implemented yet (actors, supervisor, adapters are still placeholders).

## Next State
- Address Phase 1 domain issues:
  - Move HTTP DTOs (`ChatCompletionRequest`, etc.) out of `src/core/types.rs`.
  - Make domain validation return `SentinelError` variants and protect `ApiKey` secrets.
- Complete Phase 2 implementation:
  - Implement `src/engine/actor.rs` and `src/engine/supervisor.rs` with bounded channels and `tokio::select!`.
  - Add integration tests (`tests/`) that exercise the actor loop with mock traits.
- Harden API surface:
  - Consolidate auth middleware paths, redact error responses, and hash/store API keys securely.
  - Connect `src/api/routes.rs` handlers to the engine once available; until then, gate endpoints with feature flags or explicit “not implemented” responses.
- Frontend follow-up:
  - Replace Vite starter UI with the routed shell described in `frontend/tasks/prd.md`.
  - Consume shared types from `frontend/src/types` within data services and components.
- Keep recording new findings in `docs/code-review/` as work progresses and update this bridge after each major milestone.
