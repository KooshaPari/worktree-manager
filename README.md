# worktree-manager

Git worktree automation and management toolkit with hexagonal architecture.

## Overview

`worktree-manager` provides a robust, well-tested interface for managing git worktrees across your repositories. Built with hexagonal architecture principles for maximum flexibility and testability.

## Features

- **List worktrees** - View all worktrees in a repository
- **Create worktrees** - Create new worktrees with associated branches
- **Remove worktrees** - Clean up worktrees with safety checks
- **Lock/Unlock** - Prevent accidental modifications
- **Cleanup** - Remove stale worktrees based on policies
- **Prune** - Clean up stale worktree references

## Architecture

This crate follows **Hexagonal Architecture** (Ports & Adapters):

```
src/
├── domain/           # Pure business logic (zero external deps)
│   ├── models.rs   # Worktree, BranchName, etc.
│   └── errors.rs   # Domain errors (PoLA pattern)
├── ports/           # Interface definitions
│   └── mod.rs      # Repository, BranchOperations traits
├── application/     # Use case orchestration
│   └── service.rs  # WorktreeService
└── infrastructure/  # Adapters
    ├── git_adapter.rs      # git2 implementation
    └── filesystem_adapter.rs # std::fs implementation
```

### Applied Principles

| Principle | Implementation |
|-----------|----------------|
| **SOLID** | Dependency Inversion via traits |
| **DRY** | Shared port interfaces |
| **KISS** | Simple, focused interfaces |
| **GRASP** | Application Service pattern |
| **PoLA** | Descriptive error types |

## Installation

```bash
cargo install worktree-manager
```

Or build from source:

```bash
cargo build --release
cp target/release/wtm /usr/local/bin/
```

## Usage

```bash
# List all worktrees
wtm list

# Create a new worktree
wtm create --branch feature/my-feature --path ../worktrees/my-feature

# Remove a worktree
wtm remove ../worktrees/my-feature

# Lock a worktree
wtm lock ../worktrees/my-feature --reason "in-progress"

# Unlock a worktree
wtm unlock ../worktrees/my-feature

# Clean up stale worktrees (dry run)
wtm cleanup --remove-stale --dry-run

# Show current branch
wtm branch
```

## Library Usage

```rust
use worktree_manager::{
    WorktreeService,
    domain::{BranchName, CleanupPolicy},
    infrastructure::{GitWorktreeAdapter, GitBranchAdapter, GitCleanupAdapter, SimpleFilesystemAdapter},
};

fn main() {
    let service = WorktreeService::new(
        GitWorktreeAdapter::new(),
        GitBranchAdapter::new(),
        GitCleanupAdapter::new(),
        SimpleFilesystemAdapter::new(),
    );
    
    let listing = service.list_worktrees("/path/to/repo").unwrap();
    println!("Found {} worktrees", listing.total_count);
}
```

## x-DD Methodologies

This crate applies the following methodologies:

- **Hexagonal Architecture** - Ports & Adapters isolation
- **SOLID** - Single Responsibility, Dependency Inversion
- **DRY** - Shared port interfaces
- **GRASP** - Application Service pattern
- **PoLA** - Principle of Least Astonishment (errors)

## License

MIT
