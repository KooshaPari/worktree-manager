# PRD — worktree-manager

## Overview
worktree-manager is a Git worktree lifecycle management tool for the Phenotype monorepo and multi-repo ecosystem. It provides ergonomic create, list, switch, prune, and sync operations for worktrees, with policy enforcement for the Phenotype canonical/worktree split convention.

## Epics

### E1 — Worktree Lifecycle
**E1.1** Create worktrees with a consistent naming convention (`<repo>-wtrees/<topic>`).
**E1.2** List all active worktrees with their branch and status.
**E1.3** Switch active shell context to a worktree.
**E1.4** Prune stale worktrees (merged branches, deleted refs).
**E1.5** Remove a specific worktree cleanly.

### E2 — Policy Enforcement
**E2.1** Enforce canonical folder protection: warn when user attempts to author feature work in canonical (`main`) folder.
**E2.2** Naming convention validation: reject worktree names that do not match `<repo>-wtrees/<topic>`.
**E2.3** Branch state check: warn if worktree branch is behind its upstream.

### E3 — Sync and Integration
**E3.1** Sync worktree with upstream: `worktree-manager sync <name>` rebases or merges.
**E3.2** Merge worktree back to canonical: prepare merge commit or PR.
**E3.3** Status report: show all worktrees with behind/ahead counts.

### E4 — Shell Integration
**E4.1** Shell hook: display current worktree in prompt.
**E4.2** Completion scripts for bash/zsh/fish.
**E4.3** Config file (`~/.worktree-manager.toml`) for per-user defaults.

## Acceptance Criteria
- Pruning stale worktrees is idempotent and non-destructive (does not delete unmerged branches without confirmation).
- All operations print clear status lines; no silent mutations.
- Policy warnings are printed to stderr, not suppressed.
