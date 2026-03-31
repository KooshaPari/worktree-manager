# Kilo Gastown Methodology Specification

## worktree-manager

**Version:** 1.0  
**Status:** Active  
**Rig ID:** 094f2480-bb68-4bf9-98e5-c658156443b6  
**Town ID:** 78a8d430-a206-4a25-96c0-5cd9f5caf984  
**Repository:** worktree-manager

---

## 1. Overview

Kilo Gastown is a multi-agent orchestration methodology for coordinating distributed software engineering work across ephemeral worktree-based development environments. It combines bead-based work tracking with convoy-mediated feature delivery, enabling parallel execution of tasks while maintaining coherent delivery pipelines.

This document describes how Kilo Gastown mechanics apply to the worktree-manager project within the AgilePlus+Kilo town.

---

## 2. Core Concepts

### 2.1 Rig and Town

| Concept | Definition |
|---------|------------|
| **Rig** | A shared development environment (094f2480-bb68-4bf9-98e5-c658156443b6) containing worktrees for multiple agents |
| **Town** | A coordination zone (78a8d430-a206-4a25-96c0-5cd9f5caf984) that manages bead routing and agent dispatch |
| **Worktree** | Isolated git working directory linked to the canonical repository on `main` |

### 2.2 Beads

Beads are the fundamental unit of work in Kilo Gastown:

| Bead Type | Purpose |
|-----------|---------|
| **issue** | Discrete task or bug fix |
| **convoy** | Long-running feature delivery pipeline |
| **merge_request** | Code review artifact |

**Bead Lifecycle:**

```
open → in_progress → in_review → closed
```

| State | Description |
|-------|-------------|
| `open` | Not yet started, available for assignment |
| `in_progress` | Agent is actively working |
| `in_review` | Submitted for merge/review |
| `closed` | Completed and merged |

### 2.3 Convoys

Convoys group related beads for coordinated delivery:

- A **convoy** is a delivery pipeline for a feature or epic
- Each convoy has a dedicated feature branch: `convoy/<name>/<id>/head`
- Beads are linked to a convoy via `metadata.convoy_id`
- Convoys enable parallel bead execution with coherent grouping

**Convoy Naming:**
```
convoy/<project>-<description>/<convoy-id>/head
```

Example: `convoy/agileplus-kilo-specs-worktree-manager/7c4b575b/head`

### 2.4 Agents

Agents are named computational units that process beads:

| Agent Role | Function |
|------------|----------|
| **polecat** | Primary execution agent; works beads on its hook |
| **refinery** | Merge and PR management; processes completed work |
| **patrol** | Triage and routing |
| **orchestrator** | High-level coordination |

Agent identity format: `<name>-<number>-<role>@<town>`

Example: `Polecat-25-polecat-094f2480@78a8d430`

---

## 3. Kilo Tool Reference

### 3.1 Agent Tools

| Tool | Purpose |
|------|---------|
| `gt_prime` | Get full context: agent record, hooked bead, mail, open beads |
| `gt_done` | Signal bead completion; push branch and transition to `in_review` |
| `gt_bead_status` | Inspect current state of any bead |
| `gt_bead_close` | Close a bead when fully complete and merged |
| `gt_mail_send` | Send coordination message to another agent |
| `gt_mail_check` | Read and acknowledge pending mail |
| `gt_nudge` | Real-time wake-up signal to another agent |
| `gt_escalate` | Create escalation bead for blocked issues |
| `gt_checkpoint` | Write crash-recovery data to agent record |
| `gt_status` | Emit plain-language status for dashboard |
| `gt_mol_current` | Get current molecule step for hooked bead |
| `gt_mol_advance` | Complete current step and advance molecule |

### 3.2 Delegation Mechanics

**gt_sling / gt_sling_batch** are used for delegating work to specialized agents:

- `gt_sling` - Delegate a single bead to another agent
- `gt_sling_batch` - Delegate multiple beads at once for parallel execution

Delegation enables load distribution across the agent pool.

### 3.3 Progress Tracking

**gt_list_convoys** shows delivery progress across all convoys in the town.

---

## 4. Workflow for worktree-manager

### 4.1 Starting a Session

1. Call `gt_prime` to get context
2. Review hooked bead details
3. Examine any pending mail via `gt_mail_check`
4. Begin work on the hooked bead immediately

### 4.2 Working a Bead

1. **Execute immediately** - No preamble or status announcements
2. **Write code** - Implement the feature/fix
3. **Run quality gates** - Pass all pre-submission checks
4. **Commit frequently** - After each meaningful unit of work
5. **Push after every commit** - Container is ephemeral
6. **Checkpoint** - Call `gt_checkpoint` after significant milestones

### 4.3 Pre-Submission Gates

Before calling `gt_done`, run:

```bash
cargo fmt
cargo clippy
cargo test
cargo build
```

If any gate fails, fix and re-run until passing.

### 4.4 Completing a Bead

1. Push final commit
2. Call `gt_done` with branch name
3. Bead transitions to `in_review`
4. Refinery picks up for merge

**Important:**
- Do NOT create PRs manually
- Do NOT merge into main
- Do NOT use `gh pr create` or `git merge`
- Just push branch and call `gt_done`

---

## 5. Branch Strategy

### 5.1 Worktree Discipline

- Canonical repository remains on `main`
- Each agent works in an isolated worktree
- Feature branches are created within worktrees

### 5.2 Branch Naming

| Type | Pattern | Example |
|------|---------|---------|
| Convoy | `convoy/<name>/<id>/head` | `convoy/agileplus-kilo-specs-worktree-manager/7c4b575b/head` |
| Feature | `feature/<topic>` | `feature/add-cleanup-policy` |
| Fix | `fix/<issue>` | `fix/worktree-lock-error` |

### 5.3 Convoy Feature Branches

Convoys use dedicated feature branches:
```
convoy/<project>-<description>/<convoy-id>/head
```

This allows the convoy to track multiple related beads while maintaining a coherent branch history.

---

## 6. Merge Modes

Kilo Gastown supports two merge modes:

| Mode | Description |
|------|-------------|
| **Review-then-land** | Agent pushes branch → `gt_done` → Refinery reviews → Merges |
| **Review-and-merge** | Agent pushes branch → `gt_done` → Refinery merges immediately |

The mode is determined by convoy configuration and bead metadata.

---

## 7. Escalation

When blocked or unable to resolve an issue:

1. Call `gt_escalate` with:
   - Clear description of the problem
   - What has been tried
   - Relevant context
2. Continue working on other aspects if possible
3. Wait for guidance from supervisor/mayor

---

## 8. Communication

### 8.1 Mail

- `gt_mail_send` - Formal persistent message
- `gt_mail_check` - Read pending mail (marks as delivered)

### 8.2 Nudges

- `gt_nudge` - Real-time wake-up signal for time-sensitive coordination
- Modes: `immediate`, `queue`, `wait-idle`

### 8.3 Status Updates

Call `gt_status` at meaningful phase transitions:
- Beginning a new file or significant phase
- Running tests or linting
- Pushing branch

Keep messages brief: "Writing unit tests for the API endpoints."

---

## 9. Agent Hygiene Rules

1. **Never commit agent directories** - `.claude/`, `.codex/`, `.cursor/`
2. **Never modify files outside worktree** - Stay in assigned scope
3. **Never force push or hard reset** - Respect remote history
4. **Never skip CI gates** - All checks must pass

---

## 10. Integration with AgilePlus

Kilo Gastown operates in conjunction with AgilePlus methodology:

| Aspect | AgilePlus | Kilo Gastown |
|--------|-----------|--------------|
| Architecture | Hexagonal | - |
| Work tracking | Beads | Beads (same entity) |
| Branch strategy | Worktree-based | Convoy feature branches |
| Quality gates | cargo fmt/clippy/test | cargo fmt/clippy/test |
| Error handling | PoLA | - |

The bead types (`issue`, `convoy`, `merge_request`) are the unified work representation across both methodologies.

---

## 11. Document Relationships

| Document | Purpose |
|----------|---------|
| `CLAUDE.md` | Development guidelines for AI assistants |
| `AGILEPLUS_SPEC.md` | AgilePlus methodology reference |
| `KILO_GASTOWN_SPEC.md` | This document - Kilo orchestration |
| `ADR.md` | Architecture decisions |
| `PRD.md` | Product requirements |
| `FUNCTIONAL_REQUIREMENTS.md` | Feature specifications |

---

## 12. Quick Reference

### Essential Commands

```bash
# Get context
gt_prime

# Complete a bead
gt_done --branch <branch-name>

# Check mail
gt_mail_check

# Status update
gt_status --message "Running tests before commit"

# Quality gates
cargo fmt
cargo clippy
cargo test
cargo build
```

### Bead States

```
open → in_progress → in_review → closed
```

---

*This specification is the authoritative reference for Kilo Gastown methodology as applied to worktree-manager in town 78a8d430-a206-4a25-96c0-5cd9f5caf984.*
