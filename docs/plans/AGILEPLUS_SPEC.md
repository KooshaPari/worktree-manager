# AgilePlus Methodology Specification

## worktree-manager

**Version:** 1.0  
**Status:** Active  
**Repository:** worktree-manager

---

## 1. Overview

AgilePlus is an opinionated agile methodology designed for software engineering teams building production systems. It combines agile principles with architectural discipline, emphasizing hexagonal architecture, domain-driven design, and engineering excellence. This document describes how AgilePlus is applied specifically to the worktree-manager project.

---

## 2. Core Principles

### 2.1 Hexagonal Architecture (Ports & Adapters)

The worktree-manager follows strict hexagonal architecture with clear layer isolation:

| Layer | Responsibility | Dependencies |
|-------|---------------|--------------|
| **Domain** | Pure business logic, models, errors | None (zero external deps) |
| **Ports** | Interface definitions (traits) | Domain only |
| **Application** | Use case orchestration | Domain, Ports |
| **Infrastructure** | External adapters (git, fs) | Domain, Ports |

**Note on Serialization:** `serde` and `chrono` are used exclusively in the infrastructure layer for persistence concerns (e.g., storing worktree state to disk). They do NOT pollute the domain layer—pure domain logic has zero external dependencies.

**Rationale:** This separation ensures the core business logic remains testable and independent of infrastructure concerns. The domain layer can evolve without being coupled to git implementation details.

### 2.2 SOLID Principles

| Principle | Application |
|-----------|-------------|
| **Single Responsibility** | Each module has one reason to change; `models.rs` contains only domain models |
| **Open/Closed** | New adapters extend via traits, not modification |
| **Liskov Substitution** | Any `WorktreeRepository` implementation works identically |
| **Interface Segregation** | Small, focused traits (e.g., `BranchOperations`, `CleanupOperations`) |
| **Dependency Inversion** | Application depends on traits (ports), not concrete implementations |

### 2.3 Domain-Driven Design

**Bounded Context:** Git worktree management

**Key Value Objects:**
- `WorktreeId` - PathBuf wrapper ensuring type safety
- `BranchName` - String wrapper with validation

**Aggregates:**
- `Worktree` - Core entity with path, branch, head, lock status
- `CleanupPolicy` - Encapsulates cleanup rules and thresholds
- `WorktreeListing` - Aggregates multiple worktrees for listing operations
- `WorktreeResult` - Result wrapper for worktree operations

**Domain Events (implicit):**
- Worktree created/removed/locked/unlocked
- Prune operations

---

## 3. Workflow Discipline

### 3.1 Branch Strategy

- **Canonical Repository:** Always remains on `main`
- **Worktrees for Features:** Each feature/fix uses a dedicated worktree
- **Naming Convention:** `feature/<topic>`, `fix/<issue>`, `refactor/<module>`
- **Convoy Branch Convention:** `convoy/<feature>/<convoy-id>/gt/<agent>/<bead-id>` - For coordinated multi-agent work on the same feature

### 3.2 Commit Hygiene

- **Frequency:** Commit after every meaningful unit of work
- **Message Format:** `type(scope): description` (Conventional Commits)
- **Atomic Changes:** Each commit represents one logical change
- **Push Discipline:** Push after every commit (ephemeral containers)

### 3.3 Code Quality Gates

All changes must pass before merging:

```bash
cargo fmt        # Code formatting
cargo clippy     # Lint with strict warnings
cargo test       # Unit and integration tests
cargo build      # Compilation verification
cargo doc --no-deps  # Documentation build
```

---

## 4. Error Handling (PoLA)

AgilePlus applies the **Principle of Least Astonishment** to error design:

### 4.1 Error Type Hierarchy

```rust
// Domain errors are specific and actionable
AlreadyExists        // "Worktree already exists at path"
NotFound             // "No worktree found at path"
BranchExists         // "Branch 'feature/x' already exists"
CannotModifyMain     // "Main worktree cannot be modified"
Locked               // "Worktree is locked: {reason}"
Stale                // "Worktree has stale references"
BranchNotFound       // "Branch '{branch}' not found"
UnmergedChanges      // "Worktree has unmerged changes: {path}"
InvalidBranchName    // "Invalid branch name: {name}"
InvalidPath          // "Invalid path: {path}"
GitError             // "Git operation failed: {detail}"
IoError              // "IO error: {detail}"
```

### 4.2 Error Communication

- Errors communicate **what** failed and **why**
- Recovery hints embedded in error messages
- No generic `Error` types in domain layer

---

## 5. Testing Strategy

### 5.1 Test Pyramid

| Level | Scope | Tool |
|-------|-------|------|
| **Unit** | Domain logic, value objects | `#[test]` in domain |
| **Integration** | Repository adapters | `assert_cmd`, `tempfile` |
| **CLI** | End-to-end commands | `wtm` binary invocation |

### 5.2 Test Isolation

- Unit tests have no external dependencies
- Integration tests use temporary directories
- No mocking of domain types (only ports)

---

## 6. Documentation Standards

### 6.3 Required Documentation

| Document | Purpose | Location |
|---------|---------|----------|
| `README.md` | Project overview, quick start | Root |
| `CLAUDE.md` | AI assistant development guidelines | Root |
| `AGILEPLUS_SPEC.md` | This document | `docs/plans/` |
| `ADR.md` | Architecture decision records | Root |
| `PRD.md` | Product requirements | Root |
| `FUNCTIONAL_REQUIREMENTS.md` | Detailed requirements | Root |

### 6.4 Code Documentation

- Public APIs require doc comments
- Complex algorithms include explanatory comments
- No commented-out code (version control preserves history)

---

## 7. CI/CD Pipeline

### 7.1 Pipeline Stages

```
┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐
│  fmt    │ → │ clippy  │ → │ build   │ → │  test   │ → │  doc    │
└─────────┘   └─────────┘   └─────────┘   └─────────┘   └─────────┘
```

### 7.2 Quality Gates

- **fmt:** `cargo fmt --all -- --check`
- **clippy:** `cargo clippy -- -D warnings`
- **build:** `cargo build --verbose`
- **test:** `cargo test --verbose`
- **doc:** `cargo doc --no-deps`

---

## 8. Repository Structure

```
worktree-manager/
├── src/
│   └── worktree_manager/
│       ├── domain/           # Pure business logic
│       │   ├── models.rs     # Worktree, BranchName, CleanupPolicy
│       │   └── errors.rs     # Domain errors (PoLA)
│       ├── ports/            # Interface definitions
│       │   └── mod.rs        # WorktreeRepository, BranchOperations
│       ├── application/       # Use case orchestration
│       │   └── service.rs    # WorktreeService
│       ├── infrastructure/   # Adapters
│       │   ├── git_adapter.rs
│       │   └── filesystem_adapter.rs
│       └── cli/               # CLI interface
│           └── mod.rs
├── docs/plans/               # Specs and plans
├── Cargo.toml
├── CLAUDE.md
├── ADR.md
├── PRD.md
└── FUNCTIONAL_REQUIREMENTS.md
```

---

## 9. Phenotype Org Rules

All contributors must follow these rules:

1. **UTF-8 Only:** All text files use UTF-8 encoding
2. **Worktree Discipline:** Canonical repo stays on `main`
3. **CI Completeness:** All CI failures fixed before merging
4. **Agent Hygiene:** Never commit agent directories (`.claude/`, `.codex/`, `.cursor/`)
5. **Zero Dependencies:** Domain layer has no external crate dependencies

---

## 10. Conventions

### 10.1 Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Modules | `snake_case` | `git_adapter` |
| Types | `PascalCase` | `WorktreeService` |
| Traits | `PascalCase` | `WorktreeRepository` |
| Functions | `snake_case` | `list_worktrees` |
| Constants | `SCREAMING_SNAKE` | `MAX_RETRY_ATTEMPTS` |
| Config keys | `snake_case` | `cleanup_policy` |

### 10.2 File Organization

- One module per file (mod.rs exports)
- Tests colocated in same module (unit) or `tests/` (integration)
- Documentation in `docs/` directory

---

## 11. Related Documents

| Document | Relationship |
|----------|-------------|
| `CLAUDE.md` | Development guidelines for AI assistants |
| `ADR.md` | Architecture decision rationale |
| `PRD.md` | Product vision and roadmap |
| `FUNCTIONAL_REQUIREMENTS.md` | Detailed feature specifications |

---

*This specification is the authoritative reference for AgilePlus methodology as applied to worktree-manager. Updates require approval via the standard review process.*
