//! Application layer - use case orchestration
//!
//! Following Hexagonal Architecture:
//! - Contains application services that orchestrate use cases
//! - Depends only on domain and ports
//! - Coordinates between driving and driven adapters

use crate::domain::{BranchName, DomainResult, WorktreeListing, WorktreeResult};
use crate::ports::{BranchOperations, WorktreeRepository};
use std::path::Path;

/// Application service for worktree management
pub struct WorktreeService<R, B>
where
    R: WorktreeRepository,
    B: BranchOperations,
{
    repository: R,
    branches: B,
}

impl<R, B> WorktreeService<R, B>
where
    R: WorktreeRepository,
    B: BranchOperations,
{
    pub fn new(repository: R, branches: B) -> Self {
        Self {
            repository,
            branches,
        }
    }

    /// List all worktrees in a repository
    pub fn list_worktrees(&self, repo_path: &Path) -> DomainResult<WorktreeListing> {
        self.repository.list(repo_path)
    }

    /// Create a new worktree with branch
    pub fn create_worktree(
        &self,
        repo_path: &Path,
        branch_name: BranchName,
        worktree_path: &Path,
        _start_point: Option<&str>,
    ) -> WorktreeResult {
        // Check if branch already exists
        match self.branches.exists(repo_path, &branch_name) {
            Ok(true) => {
                return WorktreeResult::failure(format!(
                    "Branch '{}' already exists",
                    branch_name.as_str()
                ))
            }
            Err(e) => return WorktreeResult::failure(e.to_string()),
            _ => {}
        }

        // Create worktree
        match self.repository.create(repo_path, &branch_name, worktree_path) {
            Ok(worktree) => WorktreeResult::success(worktree),
            Err(e) => WorktreeResult::failure(e.to_string()),
        }
    }

    /// Remove a worktree
    pub fn remove_worktree(&self, worktree_path: &Path, force: bool) -> DomainResult<()> {
        self.repository.remove(worktree_path, force)
    }

    /// Lock a worktree
    pub fn lock_worktree(&self, worktree_path: &Path, reason: &str) -> DomainResult<()> {
        self.repository.lock(worktree_path, reason)
    }

    /// Unlock a worktree
    pub fn unlock_worktree(&self, worktree_path: &Path) -> DomainResult<()> {
        self.repository.unlock(worktree_path)
    }

    /// Prune stale worktree references
    pub fn prune(&self, repo_path: &Path) -> DomainResult<()> {
        self.repository.prune(repo_path)
    }

    /// Get current branch
    pub fn current_branch(&self, repo_path: &Path) -> DomainResult<BranchName> {
        self.branches.current(repo_path)
    }
}
