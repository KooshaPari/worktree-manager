# CLAUDE.md - Development Guidelines for worktree-manager

## Project Overview

worktree-manager is a git worktree automation and management toolkit with hexagonal architecture. It provides a robust interface for managing git worktrees with safety checks and cleanup policies.

## Key Files

- `README.md` - Project overview and usage
- `src/domain/` - Pure business logic (models, errors)
- `src/ports/` - Interface definitions (Repository traits)
- `src/application/` - Use case orchestration
- `src/infrastructure/` - git2 and std::fs implementations

## Development Commands

```bash
cargo test       # Run all tests
cargo clippy     # Lint
cargo fmt        # Format
cargo build      # Build
```

## Architecture Principles

- **Hexagonal Architecture** - Ports & Adapters isolation
- **SOLID** - Single Responsibility, Dependency Inversion via traits
- **DRY** - Shared port interfaces
- **PoLA** - Descriptive error types (Principle of Least Astonishment)

## Phenotype Org Rules

- UTF-8 encoding only in all text files
- Worktree discipline: canonical repo stays on `main`
- CI completeness: fix all CI failures before merging
- Never commit agent directories (`.claude/`, `.codex/`, `.cursor/`)
- Domain layer must have ZERO external dependencies
