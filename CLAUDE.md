# CLAUDE.md - Development Guidelines for worktree-manager

## Project Overview

worktree-manager is a git worktree automation and management toolkit with hexagonal architecture. It provides a robust interface for managing git worktrees with safety checks and cleanup policies.

## Stack

- **Language**: Rust 2021 edition
- **Architecture**: Hexagonal (Ports & Adapters)
- **Core Dependencies**:
  - `anyhow` - Error handling
  - `thiserror` - Derive macros for errors
  - `serde` / `serde_json` - Serialization
  - `chrono` - Date/time with serde support
  - `clap` - CLI argument parsing (derive mode)
  - `tracing` / `tracing-subscriber` - Structured logging
- **Git Operations**: Subprocess `git` commands (not git2 crate)
- **Dev Dependencies**: `tempfile`, `assert_cmd`

## Key Files

- `README.md` - Project overview and usage
- `AGENTS.md` - Kilo Gastown mechanics for this rig
- `src/main.rs` - Binary entry point (`wtm`)
- `src/worktree_manager/domain/` - Pure business logic (models, errors)
- `src/worktree_manager/ports/` - Interface definitions (Repository traits)
- `src/worktree_manager/application/` - Use case orchestration (WorktreeService)
- `src/worktree_manager/infrastructure/` - git subprocess and filesystem implementations
- `src/worktree_manager/cli/` - CLI commands

## Development Commands

```bash
cargo build       # Build the project
cargo test        # Run all tests
cargo clippy      # Lint with Clippy
cargo fmt         # Format code
```

## Architecture Principles

- **Hexagonal Architecture** - Ports & Adapters isolation
- **SOLID** - Single Responsibility, Dependency Inversion via traits
- **DRY** - Shared port interfaces
- **PoLA** - Descriptive error types (Principle of Least Astonishment)
- **Domain Layer** - Must have ZERO external dependencies

## Code Conventions

- UTF-8 encoding only in all text files
- No comments unless requested
- Use descriptive error types following PoLA pattern
- Git operations via subprocess (not git2 crate)
- Worktree discipline: canonical repo stays on `main`

## Agent Behavior Rules

- **GUPP Principle**: Work is on your hook — execute immediately
- **Commit Frequently**: Push after every meaningful unit of work
- **No Destructive Ops**: Never force push, hard reset, or merge to main
- **No PR Creation**: The Refinery handles merging; just push and call `gt_done`
- **Pre-Submission Gates**: Run `task quality` before calling `gt_done`
- **Checkpoint**: Call `gt_checkpoint` after significant milestones

## Phenotype Org Rules

- UTF-8 encoding only in all text files
- Worktree discipline: canonical repo stays on `main`
- CI completeness: fix all CI failures before merging
- Never commit agent directories (`.claude/`, `.codex/`, `.cursor/`)
- Domain layer must have ZERO external dependencies
