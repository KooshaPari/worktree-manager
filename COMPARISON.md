# Comparison Matrix: Git Worktree Tools

## Feature Comparison

| Feature | worktree-manager | git-worktree CLI | gh worktree | go-git worktree |
|---------|----------------|-----------------|-------------|-----------------|
| **List worktrees** | ✅ | ✅ | ✅ | ❌ |
| **Create worktree** | ✅ | ✅ | ✅ | ❌ |
| **Remove worktree** | ✅ | ✅ | ✅ | ❌ |
| **Lock/Unlock** | ✅ | ❌ | ❌ | ❌ |
| **Cleanup policy** | ✅ | ❌ | ❌ | ❌ |
| **Prune stale** | ✅ | ✅ | ❌ | ❌ |
| **JSON output** | ✅ | ❌ | ❌ | ❌ |
| **Multi-repo** | ✅ | ❌ | ❌ | ❌ |
| **Batch operations** | ✅ | ❌ | ❌ | ❌ |

## Architecture Comparison

| Aspect | worktree-manager | git-worktree CLI | gh worktree |
|--------|-----------------|-----------------|-------------|
| **Architecture** | Hexagonal | Monolithic | Monolithic |
| **Language** | Rust | C | Go |
| **Testable** | ✅ High | ❌ Low | ⚠️ Medium |
| **Extensible** | ✅ Via ports | ❌ No | ⚠️ Limited |
| **Library** | ✅ Yes | ❌ No | ❌ No |
| **CLI** | ✅ Yes | ✅ Yes | ✅ Yes |

## Use Case Comparison

| Use Case | Recommended Tool |
|----------|-----------------|
| Simple worktree listing | git worktree CLI |
| GitHub-integrated workflow | gh worktree |
| **Enterprise automation** | **worktree-manager** |
| **Policy-based cleanup** | **worktree-manager** |
| **Custom integrations** | **worktree-manager** |

## Why worktree-manager?

1. **Hexagonal Architecture** - Fully testable, pluggable adapters
2. **Policy-based cleanup** - Automated stale worktree detection
3. **Lock mechanism** - Prevent accidental modifications
4. **Library + CLI** - Use as library or CLI tool
5. **Rust-powered** - Memory-safe, fast, zero dependencies at runtime

## Alternatives Considered

| Tool | Pros | Cons |
|------|------|------|
| git-worktree | Built-in, fast | No policy, no library |
| gh worktree | GitHub integration | GitHub-specific, no policy |
| grm | Rust-based | No hexagonal, limited features |
| git-batch | Batch operations | No worktree-specific features |

## Value Proposition

`worktree-manager` fills the gap between raw git worktree commands and enterprise automation needs:

- **For CLI users**: More safety (lock/unlock) and better output (JSON)
- **For automation**: Policy-based cleanup, library API
- **For enterprises**: Audit trails, compliance features
