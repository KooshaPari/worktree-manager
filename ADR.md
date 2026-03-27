# ADR — worktree-manager

## ADR-001: Worktree Path Convention
**Status:** Accepted
**Context:** Multiple repos and multiple agents create worktrees; a consistent path schema prevents conflicts.
**Decision:** Worktree paths follow `<repo-root>/../<repo-name>-wtrees/<topic-slug>/`. The canonical repo remains at `<repo-name>/`.
**Rationale:** Keeps worktrees sibling to canonical, easy to glob, and clearly separated from source.

## ADR-002: Shell Context Switching
**Status:** Accepted
**Context:** `cd` cannot be run inside a subprocess; the tool must integrate with the calling shell.
**Decision:** The `switch` subcommand prints `cd <path>` to stdout; users eval via `eval $(worktree-manager switch <name>)`. Alternatively, a shell function wrapper is provided.
**Rationale:** Standard pattern for shell-integrated tools (e.g., `direnv`, `nvm`).

## ADR-003: Prune Safety
**Status:** Accepted
**Context:** Pruning must not silently destroy unmerged work.
**Decision:** Prune checks merge status against `origin/main` before deletion. Unmerged worktrees require explicit `--force`. Interactive prompt for merged worktrees unless `--yes`.
**Rationale:** Fail-safely; data loss requires explicit user intent.

## ADR-004: Config Format
**Status:** Accepted
**Context:** Per-user defaults (naming pattern, default branch, prune policy) should be configurable.
**Decision:** `~/.worktree-manager.toml` for user config; `.worktree-manager.toml` in repo root for project config. Project config takes precedence.
**Rationale:** TOML is human-friendly; two-tier config allows per-project overrides.
