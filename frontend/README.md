# Sentinel Orchestrator Frontend

React + TypeScript SPA that visualizes the Sentinel backend. Follow the PRD in `tasks/prd.md` and keep all TypeScript contracts synchronized with `src/core/types.rs`.

## Tooling
- **Package Manager:** Bun only. Never use npm/yarn/pnpm in this workspace.
- **Runtime:** Vite + React 19 with ESLint 9 and TypeScript 5.

Install dependencies once per machine:

```bash
bun install
```

## Development Commands

| Task | Command | Notes |
| --- | --- | --- |
| Dev server with HMR | `bun run dev` | Serves at http://localhost:5173 |
| Type check + production build | `bun run build` | Runs `tsc -b` followed by `vite build` |
| Preview prod build | `bun run preview` | Uses Vite preview server |
| Lint | `bun run lint` | Must pass before PR submission |

## Project Structure
- `src/` – React components, routes, hooks, and shared types (`src/types` mirrors Rust core types).
- `public/` – Static assets copied verbatim to the build.
- `tasks/` – Frontend-specific bridge + PRD; keep them updated as flows evolve.
- `bun.lock` – Deterministic dependency lock file (do not edit manually).

## Workflow Expectations
1. Model changes in Rust first, regenerate shared TypeScript types, then consume them in React.
2. Follow the neo-punk design language documented in `tasks/prd.md`.
3. Use Bun scripts for all CI commands and Docker builds (see `Dockerfile` for reference).
4. Record architectural or UX issues in `docs/code-review/` to keep backend + frontend reviews aligned.
