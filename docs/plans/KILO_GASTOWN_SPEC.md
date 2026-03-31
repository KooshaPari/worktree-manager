# Kilo Gastown Methodology Specification

## worktree-manager

**Version:** 1.0  
**Status:** Active  
**Rig ID:** 094f2480-bb68-4bf9-98e5-c658156443b6  
**Town:** 78a8d430-a206-4a25-96c0-5cd9f5caf984

---

## 1. Overview

Kilo Gastown is an agent orchestration methodology for coordinating distributed AI agents (polecats) across a shared rig. It provides structured patterns for work delegation, progress tracking, and quality assurance through a bead-based workflow system.

This document describes how Kilo Gastown mechanics apply specifically to the worktree-manager project.

---

## 2. Core Concepts

### 2.1 Rig and Town

| Concept | Definition |
|---------|------------|
| **Rig** | A coordinated group of agents sharing a common goal (rig ID: `094f2480-bb68-4bf9-98e5-c658156443b6`) |
| **Town** | A namespace/orchestration layer (`78a8d430-a206-4a25-96c0-5cd9f5caf984`) |
| **Polecat** | An autonomous agent that executes assigned beads |

The worktree-manager rig contains multiple agents coordinated toward the shared objective of building and maintaining the git worktree management toolkit.

### 2.2 Beads

Beads are the fundamental work unit in Kilo Gastown:

| Bead Type | Purpose |
|-----------|---------|
| **issue** | A discrete task or feature to implement |
| **merge_request** | A code review or PR in progress |
| **convoy** | A tracked group of related beads |

**Bead Lifecycle:**

```
open → in_progress → in_review → closed
```

| State | Description |
|-------|-------------|
| `open` | Not yet assigned; available for pickup |
| `in_progress` | Assigned to an agent; actively being worked |
| `in_review` | Code submitted for review; awaiting merge |
| `closed` | Completed and merged |

---

## 3. Convoys

### 3.1 What Are Convoys

Convoys are tracked groups of related beads that move together through the workflow. They enable:

- **Grouped progress tracking** - Monitor entire features/epics at once
- **Coordinated merges** - All beads in a convoy land together
- **Delegation scope** - gt_sling operates on convoy boundaries

### 3.2 Convoy Naming

Convoys use a structured naming pattern:

```
convoy/<feature-area>/<convoy-id>/head
```

**Examples:**

- `convoy/methodology-worktree-manager/7da62ebe/head`
- `convoy/agileplus-kilo-specs-worktree-manager/7c4b575b/head`

### 3.3 Convoys in worktree-manager

Current open convoys in this rig:

| Convoy ID | Feature Area |
|-----------|--------------|
| `7da62ebe-72ef-4b92-81f8-6fd266792d8f` | Methodology documentation |
| `7c4b575b-a570-4150-9ee4-5fa381b0a1c5` | AgilePlus + Kilo specs |
| `22e54005-cb4a-4188-a83b-d9140b72a617` | portage |
| `381d5195-27f8-4843-9efe-62f11234815e` | Dino |

---

## 4. Agent Tools

### 4.1 Core Agent Tools

| Tool | Purpose |
|------|---------|
| `gt_prime` | Get full context: identity, hooked bead, mail, open beads |
| `gt_done` | Signal work complete; push branch; transition to in_review |
| `gt_bead_status` | Inspect current state of any bead |
| `gt_bead_close` | Close a bead when work is fully complete |
| `gt_mail_send` | Send coordination message to another agent |
| `gt_mail_check` | Read undelivered mail addressed to this agent |
| `gt_checkpoint` | Write crash-recovery data for session resume |
| `gt_status` | Emit plain-language status update for dashboard |
| `gt_escalate` | Create escalation bead for blocked issues |

### 4.2 Delegation Tools

| Tool | Purpose |
|------|---------|
| `gt_sling` | Delegate a single bead to another agent |
| `gt_sling_batch` | Delegate multiple beads at once |

### 4.3 Convoy Tools

| Tool | Purpose |
|------|---------|
| `gt_list_convoys` | List all convoys with progress status |

---

## 5. Workflow Patterns

### 5.1 Standard Bead Flow

```
1. Agent calls gt_prime to get context
2. Agent receives hooked bead (in_progress issue)
3. Agent implements the feature
4. Agent commits and pushes frequently
5. Agent calls gt_done to transition to in_review
6. Refinery picks up for merge
7. Bead transitions to closed
```

### 5.2 Pre-Submission Gates

Before calling `gt_done`, agents must verify:

```bash
cargo fmt        # Code formatting
cargo clippy     # Lint with strict warnings
cargo test       # Unit and integration tests
cargo build     # Compilation verification
```

### 5.3 Merge Modes

Kilo supports two merge strategies:

| Mode | Description |
|------|-------------|
| **Review-then-land** | Agent pushes branch; gt_done transitions to in_review; refinery merges |
| **Review-and-merge** | Agent calls gt_done; immediate merge after review approval |

---

## 6. Coordination Patterns

### 6.1 Agent Communication

**Mail (persistent):**
- Use `gt_mail_send` for formal coordination messages
- Messages are queued and delivered on recipient's next call to `gt_mail_check`

**Nudges (immediate):**
- Use `gt_nudge` for time-sensitive coordination
- Delivered at agent's next idle moment

### 6.2 Escalation

When blocked or unable to resolve:

```bash
gt_escalate(title="Issue description", body="What was tried", priority="high")
```

### 6.3 Checkpointing

After significant milestones, save state:

```bash
gt_checkpoint(data="Implemented feature X; working on Y; branch at commit abc123")
```

---

## 7. Integration with worktree-manager

### 7.1 Existing Documentation

This Kilo Gastown spec complements existing worktree-manager documentation:

| Document | Focus |
|----------|-------|
| `CLAUDE.md` | Development guidelines for AI assistants (includes architecture principles, Phenotype Org rules) |
| `AGILEPLUS_SPEC.md` | AgilePlus methodology (hexagonal, SOLID, PoLA) |
| `KILO_GASTOWN_SPEC.md` | Agent orchestration and coordination |

Note: `AGENTS.md` is not present in this repo; all agent guidance is consolidated in `CLAUDE.md`.

### 7.2 Phenotype Org Rules (from CLAUDE.md)

Kilo Gastown agents operating in worktree-manager must follow:

1. **UTF-8 encoding** in all text files
2. **Worktree discipline** - canonical repo stays on `main`
3. **CI completeness** - fix all CI failures before merging
4. **Agent hygiene** - never commit `.claude/`, `.codex/`, `.cursor/`
5. **Zero domain deps** - domain layer has no external crate dependencies

### 7.3 Branch Strategy

Each bead gets its own worktree:

```bash
# Create worktree for new bead
git worktree add ../wt-feature-x feature/x

# Worktree discipline: canonical repo stays on main
git worktree list
```

### 7.4 Development Commands

| Command | Purpose |
|---------|---------|
| `cargo test` | Run all tests |
| `cargo clippy` | Lint |
| `cargo fmt` | Format |
| `cargo build` | Build |

---

## 8. Quality Assurance

### 8.1 Pre-Merge Gates

All code must pass:

```
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo build --verbose
cargo test --verbose
```

### 8.2 Review Process

1. Agent pushes branch to remote
2. Agent calls `gt_done` with branch name
3. Bead transitions to `in_review`
4. Refinery picks up for merge review
5. If changes requested: rework bead, push fixes, re-review
6. If approved: merge to target branch

---

## 9. Reference

### 9.1 Bead Structure

```json
{
  "bead_id": "84d72527-9baa-4a44-a74e-d30d99516bb0",
  "type": "issue",
  "status": "in_progress",
  "title": "Add Kilo Gastown methodology spec",
  "rig_id": "094f2480-bb68-4bf9-98e5-c658156443b6",
  "assignee_agent_bead_id": "5c1c6a82-0e63-4f56-b76d-6cccc5bb9215",
  "priority": "medium"
}
```

### 9.2 Agent Identity

Each polecat has a unique identity:

```
Blaze-polecat-094f2480@78a8d430
```

Format: `{name}-polecat-{rig_id}@{town_id}`

---

## 10. Related Documents

| Document | Location | Purpose |
|----------|----------|---------|
| CLAUDE.md | Root | Development guidelines (architecture, Phenotype Org rules) |
| AGILEPLUS_SPEC.md | docs/plans/ | AgilePlus methodology |
| ADR.md | Root | Architecture decisions |
| PRD.md | Root | Product requirements |

---

*This specification is the authoritative reference for Kilo Gastown methodology as applied to worktree-manager. Updates require approval via the standard review process.*
