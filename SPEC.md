# worktree-manager — Technical Specification

## Architecture (Hexagonal)

```
┌─────────────────────────────────────────────────────┐
│                  CLI (clap derive)                    │
│        list | create | remove | lock | unlock         │
│              cleanup | prune | branch                 │
├─────────────────────────────────────────────────────┤
│              Application Layer                        │
│            WorktreeService (use cases)                │
├────────────┬────────────────────────────────────────┤
│   Ports    │          Domain (pure)                  │
│  (traits)  │   Worktree, BranchName, CleanupPolicy   │
│            │   errors (PoLA pattern)                 │
│            │   ZERO external dependencies            │
├────────────┴────────────────────────────────────────┤
│              Infrastructure (Adapters)                │
│   GitWorktreeAdapter   │  SimpleFilesystemAdapter    │
│   GitBranchAdapter     │  GitCleanupAdapter          │
│   (subprocess git)     │  (std::fs)                  │
└─────────────────────────────────────────────────────┘
```

## Components

| Component | Location | Responsibility |
|-----------|----------|---------------|
| Domain | `src/worktree_manager/domain/` | Pure business logic, zero deps |
| Models | `src/worktree_manager/domain/models.rs` | Worktree, BranchName, CleanupPolicy |
| Errors | `src/worktree_manager/domain/errors.rs` | Descriptive error types |
| Ports | `src/worktree_manager/ports/mod.rs` | Repository, BranchOperations traits |
| Service | `src/worktree_manager/application/service.rs` | Use case orchestration |
| Git Adapter | `src/worktree_manager/infrastructure/git_adapter.rs` | git subprocess adapter |
| FS Adapter | `src/worktree_manager/infrastructure/filesystem_adapter.rs` | std::fs adapter |
| CLI | `src/worktree_manager/cli/mod.rs` | Command implementations |

## Domain Model

```rust
struct Worktree {
    path: PathBuf,
    branch: BranchName,
    head: CommitHash,
    locked: bool,
    prune_expiry: Option<DateTime>,
}

struct BranchName(String); // validated: no spaces, no double-dots

struct CleanupPolicy {
    remove_stale: bool,
    max_age: Option<Duration>,
    dry_run: bool,
}

struct WorktreeListing {
    worktrees: Vec<Worktree>,
    total_count: usize,
    locked_count: usize,
}
```

## Port Interfaces

```rust
trait WorktreeRepository {
    fn list(&self, repo_path: &Path) -> Result<WorktreeListing>;
    fn create(&self, repo_path: &Path, branch: &BranchName, path: &Path) -> Result<Worktree>;
    fn remove(&self, path: &Path, force: bool) -> Result<()>;
    fn lock(&self, path: &Path, reason: &str) -> Result<()>;
    fn unlock(&self, path: &Path) -> Result<()>;
    fn prune(&self, repo_path: &Path) -> Result<()>;
}

trait BranchOperations {
    fn current(&self, repo_path: &Path) -> Result<BranchName>;
    fn exists(&self, repo_path: &Path, branch: &BranchName) -> Result<bool>;
}

trait CleanupOperations {
    fn cleanup(&self, repo_path: &Path, policy: &CleanupPolicy) -> Result<CleanupResult>;
}
```

## CLI Commands

| Command | Flags | Purpose |
|---------|-------|---------|
| `list` | | List all worktrees |
| `create` | `--branch`, `--path` | Create worktree with branch |
| `remove` | `<path>` | Remove worktree |
| `lock` | `<path> --reason` | Lock worktree |
| `unlock` | `<path>` | Unlock worktree |
| `cleanup` | `--remove-stale --dry-run` | Clean stale worktrees |
| `prune` | | Prune stale references |
| `branch` | | Show current branch |

## Design Principles

| Principle | Implementation |
|-----------|---------------|
| SOLID | Dependency Inversion via trait ports |
| DRY | Shared port interfaces |
| KISS | Simple, focused interfaces |
| GRASP | Application Service pattern |
| PoLA | Descriptive error types with context |

## Performance Targets

| Metric | Target |
|--------|--------|
| List worktrees | <200ms |
| Create worktree | <2s |
| Remove worktree | <1s |
| Cleanup scan | <500ms |
| Lock/unlock | <100ms |
| Binary size | <5MB |

## Dependencies

| Crate | Purpose |
|-------|---------|
| `anyhow` | Error handling |
| `thiserror` | Error derive macros |
| `serde` / `serde_json` | Serialization |
| `chrono` | Date/time |
| `clap` | CLI parsing (derive) |
| `tracing` | Structured logging |
| `tempfile` | Test fixtures |
| `assert_cmd` | CLI testing |
