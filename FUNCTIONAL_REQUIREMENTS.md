# FUNCTIONAL_REQUIREMENTS — worktree-manager

## FR-LIFE-001: Create Worktree
**SHALL** create a new worktree at `<repo-root>/../<repo>-wtrees/<topic>/` from a branch name.
Traces to: E1.1

## FR-LIFE-002: List Worktrees
**SHALL** output a table of all worktrees: path, branch, HEAD SHA, ahead/behind counts.
Traces to: E1.2

## FR-LIFE-003: Switch Context
**SHALL** emit a `cd` command (via eval wrapper) to switch the shell to the specified worktree.
Traces to: E1.3

## FR-LIFE-004: Prune Stale
**SHALL** identify worktrees whose branch is merged to main and prompt before deletion; delete without prompt only with `--force` flag.
Traces to: E1.4

## FR-LIFE-005: Remove Worktree
**SHALL** run `git worktree remove` and clean up the directory; fail with error if untracked changes exist unless `--force`.
Traces to: E1.5

## FR-POLICY-001: Canonical Protection
**SHALL** detect when the current directory is the canonical repo folder (branch == main, no `-wtrees-` in path) and print a warning to stderr when `create` or `sync` is invoked from it.
Traces to: E2.1

## FR-POLICY-002: Naming Convention
**SHALL** reject worktree names not matching `^[a-z0-9-]+-wtrees/[a-z0-9._/-]+$` with a descriptive error.
Traces to: E2.2

## FR-SYNC-001: Sync Worktree
**SHALL** rebase (default) or merge (with `--merge` flag) worktree branch onto upstream main.
Traces to: E3.1

## FR-SYNC-002: Status Report
**SHALL** output ahead/behind counts per worktree relative to `origin/main`.
Traces to: E3.3

## FR-SHELL-001: Completions
**SHALL** generate completion scripts for bash, zsh, and fish via `worktree-manager completions <shell>`.
Traces to: E4.2
