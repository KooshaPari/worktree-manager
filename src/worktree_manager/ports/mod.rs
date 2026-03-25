//! Ports - interface definitions following Hexagonal Architecture
//!
//! These are abstractions (traits) that define how the domain interacts
//! with the outside world. Infrastructure adapters implement these ports.

use crate::domain::{BranchName, CleanupPolicy, DomainResult, Worktree, WorktreeListing};
use std::path::Path;

/// Port for git worktree operations
pub trait WorktreeRepository: Send + Sync {
    /// List all worktrees in a repository
    fn list(&self, repo_path: &Path) -> DomainResult<WorktreeListing>;

    /// Create a new worktree with associated branch
    fn create(&self, repo_path: &Path, branch: &BranchName, path: &Path) -> DomainResult<Worktree>;

    /// Remove a worktree
    fn remove(&self, worktree_path: &Path, force: bool) -> DomainResult<()>;

    /// Lock a worktree
    fn lock(&self, worktree_path: &Path, reason: &str) -> DomainResult<()>;

    /// Unlock a worktree
    fn unlock(&self, worktree_path: &Path) -> DomainResult<()>;

    /// Prune stale worktree references
    fn prune(&self, repo_path: &Path) -> DomainResult<()>;
}

/// Port for branch operations
pub trait BranchOperations: Send + Sync {
    /// Check if branch exists
    fn exists(&self, repo_path: &Path, branch: &BranchName) -> DomainResult<bool>;

    /// Create a new branch
    fn create(&self, repo_path: &Path, branch: &BranchName, from_ref: Option<&str>) -> DomainResult<()>;

    /// Delete a branch
    fn delete(&self, repo_path: &Path, branch: &BranchName) -> DomainResult<()>;

    /// Get current branch name
    fn current(&self, repo_path: &Path) -> DomainResult<BranchName>;
}

/// Port for cleanup operations
pub trait CleanupOperations: Send + Sync {
    /// Find worktrees matching cleanup policy
    fn find_for_cleanup(&self, repo_path: &Path, policy: &CleanupPolicy) -> DomainResult<Vec<Worktree>>;

    /// Clean up worktrees based on policy
    fn cleanup(&self, repo_path: &Path, policy: &CleanupPolicy) -> DomainResult<Vec<Worktree>>;
}

/// Port for filesystem operations
pub trait FilesystemOperations: Send + Sync {
    /// Check if path exists
    fn exists(&self, path: &Path) -> bool;

    /// Create directory
    fn create_dir(&self, path: &Path) -> DomainResult<()>;

    /// Remove directory
    fn remove_dir(&self, path: &Path) -> DomainResult<()>;

    /// Check if path is empty
    fn is_empty(&self, path: &Path) -> DomainResult<bool>;
}
