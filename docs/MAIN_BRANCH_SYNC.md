# Main Branch Synchronization Process

## Overview

The main branch of this repository is updated periodically with new requirements, architecture changes, and updates to the PRD. This document outlines the process for keeping the `feature/init` branch synchronized with main.

## Sync Workflow

### 1. Before Starting Work

Always check for updates from main before beginning work:

```bash
# Fetch latest from remote
git fetch origin

# Check current branch
git branch --show-current

# If on feature/init, check what's new on main
git log HEAD..origin/main --oneline

# Review changes
git diff HEAD..origin/main
```

### 2. Periodic Sync Process

During active development, sync with main at least daily:

```bash
# Ensure you're on feature/init
git checkout feature/init

# Fetch latest
git fetch origin

# Merge main into feature/init
git merge origin/main

# Resolve any conflicts if they occur
# Test that everything still compiles
cargo check

# Commit merge if successful
git commit -m "Sync with main: [brief description of updates]"
```

### 3. Handling Conflicts

If conflicts occur during merge:

1. **Review conflict files**:
   ```bash
   git status
   ```

2. **Open conflicted files** and resolve:
   - Keep feature/init changes where they don't conflict
   - Integrate main branch updates where appropriate
   - Preserve architectural decisions from feature/init

3. **Test resolution**:
   ```bash
   cargo check
   cargo test
   ```

4. **Complete merge**:
   ```bash
   git add <resolved-files>
   git commit
   ```

### 4. Updating Documentation

When main branch updates affect:

- **PRD changes**: Update `docs/prd.md` (if it exists)
- **Architecture changes**: Update `docs/architecture.md`
- **Requirements changes**: Update this research document and related docs

### 5. Automated Sync (Optional)

For frequent syncing, consider a helper script:

```bash
#!/bin/bash
# sync-main.sh

set -e

echo "Syncing feature/init with origin/main..."

git fetch origin
git checkout feature/init
git merge origin/main

echo "Running cargo check..."
cargo check

echo "Sync complete!"
```

Make executable: `chmod +x sync-main.sh`

## When to Sync

### Required Sync Points

1. **Daily**: If working actively on the feature
2. **Before major commits**: Ensure you're building on latest
3. **Before PR**: Always sync before creating pull request
4. **After main updates**: When you know main has been updated

### Sync Indicators

Watch for these signs that main has updates:

- Team announcements
- PR merges to main
- Commit notifications
- Documentation updates

## Conflict Resolution Guidelines

### Priority Order

1. **Architecture Rules**: Maintain hexagonal architecture principles
2. **Core Domain Purity**: Never break `src/core/` independence
3. **Canonical Message Model**: Preserve domain model boundaries
4. **Feature Goals**: Keep feature/init objectives intact

### Common Conflict Scenarios

**Scenario 1: New dependencies in Cargo.toml**
- Merge dependency additions
- Verify they don't break core independence

**Scenario 2: Architecture changes**
- Review changes against hexagonal principles
- Integrate compatible changes
- Document incompatibilities

**Scenario 3: Documentation updates**
- Merge documentation changes
- Update feature-specific docs accordingly

## Verification After Sync

Always verify after syncing:

```bash
# 1. Compilation check
cargo check

# 2. Test suite
cargo test

# 3. Linting
cargo clippy -- -D warnings

# 4. Format check
cargo fmt --check
```

## Rollback Procedure

If sync breaks the branch:

```bash
# View recent commits
git log --oneline -10

# Reset to before merge
git reset --hard HEAD~1

# Or reset to specific commit
git reset --hard <commit-hash>

# Force push if needed (use with caution)
# git push origin feature/init --force
```

**Warning**: Only force push if you're the sole developer on the branch.

## Branch Status Tracking

Keep track of sync status:

```bash
# Check how many commits behind/ahead
git rev-list --left-right --count feature/init...origin/main

# View unmerged commits
git log feature/init..origin/main --oneline

# View commits not in main
git log origin/main..feature/init --oneline
```

## Best Practices

1. **Small, frequent syncs** are better than large, infrequent ones
2. **Test after every sync** to catch issues early
3. **Document significant conflicts** for team awareness
4. **Communicate** when merging large changes from main
5. **Keep feature branch focused** - don't let main changes distract from feature goals

## Related Documents

- [Architecture Documentation](./architecture.md)
- [Research: Rust Orchestrators](./research/rust_orchestrators_comparison.md)
- [Build Plan](./cursor_plan.md)

---

**Last Sync**: [Update this when syncing]
**Next Scheduled Sync**: [Set reminder]

