# Task Instructions

**AUTHENTICATION IMPLEMENTED**: API key authentication and authorization system has been implemented (Phase 5). Features include:
- Core auth domain types (ApiKeyId, ApiKey, AuthLevel, AuthResult) in `src/core/auth.rs`
- Authentication middleware with Bearer token support (OpenAI-compatible)
- Authorization middleware with permission levels (Read/Write/Admin)
- API routes protected: `/v1/chat/completions` (Write), `/v1/agents/status` (Read)
- Environment variable support: `SENTINEL_API_KEY_<ID>=<KEY>:<LEVEL>`
- Comprehensive tests for all auth functionality
- See `tasks/bridge_auth.md` for full implementation details

Always start a new feature branch for new work, write tests to start, develop a phase of @tasks/prd.md - test, commit, merge, push main. Then start again. Run "/check" if in rust and use the rs_agent in claude code. Follow the PRD and only edit do not delete. If you find errors record them in tasks/errors.md

Always follow @docs/architecture.md to ensure architecture is rigorous and defined. All code must adhere to the architectural principles documented there.

## Task Files

**IMPORTANT**: Before starting work, check for numbered task files in `@tasks/` directory (e.g., `task-2.md`, `task-3.md`, etc.).

### Task File Workflow

1. **Check for Task Files**: Look for `task-*.md` files in `@tasks/` directory
2. **Read Task Dependencies**: Each task file lists its dependencies (e.g., "REQUIRES Task 2" or "REQUIRES Phase 1")
3. **Verify Dependencies**: Ensure all dependencies are complete before starting a task
4. **Follow Task Instructions**: Each task file contains:
   - Specific objectives
   - Implementation tasks with code structure
   - Testing requirements
   - Acceptance criteria
   - References to PRD and architecture docs
5. **Work Sequentially**: Complete tasks in numerical order (task-2.md → task-3.md → task-4.md, etc.)
6. **Update Bridge**: After completing a task, update `@tasks/bridge.md` with current/next state

### Task File Structure

Each task file follows this structure:
- **Dependencies**: What must be completed first
- **Objectives**: High-level goals
- **Implementation Tasks**: Specific code to write
- **Testing Requirements**: Test cases to implement
- **Acceptance Criteria**: Checklist for completion
- **References**: Links to PRD and architecture docs

### Example Workflow

1. Check `@tasks/task-2.md` exists
2. Read dependencies: "REQUIRES Phase 1 (Core Domain)"
3. Verify Phase 1 is complete
4. Create feature branch: `feature/phase-2-task-2-channels`
5. Write tests first (as specified in task file)
6. Implement code (following task file structure)
7. Run tests and verify acceptance criteria
8. Commit, merge, push
9. Move to `task-3.md` (checking its dependencies first)

### If No Task Files Exist

If no `task-*.md` files exist for the current phase:
- Follow the PRD phase requirements directly
- Break down the phase into logical tasks
- Create task files for future work (optional)
- Follow the standard workflow from `@tasks/start.md`

