# AGENTS.md - Kilo Gastown Mechanics for worktree-manager

## Kilo Gastown Identity

- **Rig ID**: `094f2480-bb68-4bf9-98e5-c658156443b6`
- **Town**: `78a8d430-a206-4a25-96c0-5cd9f5caf984`
- **Role**: Polecat (Blaze)

## Kilo Integration

### Work Delegation

Use `gt_sling` and `gt_sling_batch` for delegating work to other agents in the rig:

```bash
gt_sling <agent-id> <task>
gt_sling_batch <batch-file>
```

### Communication

- **gt_mail_send** - Send formal persistent messages to other agents
- **gt_nudge** - Real-time coordination for time-sensitive matters
- **gt_mail_check** - Check for undelivered mail periodically

### Workflow

1. **Prime**: Call `gt_prime` at session start to get hooked bead context
2. **Work**: Implement bead requirements with small, focused commits
3. **Checkpoint**: Call `gt_checkpoint` after significant milestones
4. **Done**: Push branch and call `gt_done` when complete

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

## Build & Test Commands

```bash
cargo build       # Build the project
cargo test        # Run all tests
cargo clippy      # Lint with Clippy
cargo fmt         # Format code
```

## Project Structure

```
src/
├── main.rs                           # Binary entry point (wtm)
└── worktree_manager/
    ├── mod.rs                        # Library root
    ├── domain/                       # Pure business logic (zero deps)
    │   ├── models.rs                 # Worktree, BranchName, etc.
    │   └── errors.rs                 # Domain errors (PoLA pattern)
    ├── ports/                        # Interface definitions
    │   └── mod.rs                    # Repository, BranchOperations traits
    ├── application/                 # Use case orchestration
    │   └── service.rs                # WorktreeService
    ├── infrastructure/              # Adapters
    │   ├── git_adapter.rs            # git subprocess implementation
    │   └── filesystem_adapter.rs     # std::fs implementation
    └── cli/
        └── mod.rs                    # CLI commands
```

## Agent Behavior Rules

- **GUPP Principle**: Work is on your hook — execute immediately
- **Commit Frequently**: Push after every meaningful unit of work
- **No Destructive Ops**: Never force push, hard reset, or merge to main
- **No PR Creation**: The Refinery handles merging; just push and call `gt_done`
- **Pre-Submission Gates**: Run `task quality` before calling `gt_done`

## Phenotype Org Rules

- UTF-8 encoding only in all text files
- Worktree discipline: canonical repo stays on `main`
- CI completeness: fix all CI failures before merging
- Never commit agent directories (`.claude/`, `.codex/`, `.cursor/`)
- Domain layer must have ZERO external dependencies
