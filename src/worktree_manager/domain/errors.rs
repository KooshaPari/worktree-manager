//! Domain errors following PoLA (Principle of Least Astonishment)
//!
//! All errors are descriptive and specific, following hexagonal architecture.

use thiserror::Error;

/// Domain errors that can occur during worktree operations
#[derive(Error, Debug)]
pub enum WorktreeError {
    /// Worktree already exists at the specified path
    #[error("Worktree already exists at {0}")]
    AlreadyExists(String),

    /// Worktree does not exist
    #[error("Worktree not found at {0}")]
    NotFound(String),

    /// Branch already exists
    #[error("Branch '{0}' already exists")]
    BranchExists(String),

    /// Branch does not exist
    #[error("Branch '{0}' not found")]
    BranchNotFound(String),

    /// Cannot operate on main working directory
    #[error("Cannot perform this operation on the main working directory")]
    CannotModifyMain,

    /// Worktree is locked
    #[error("Worktree is locked: {reason}")]
    Locked { reason: String },

    /// Worktree has unmerged changes
    #[error("Worktree has unmerged changes, refusing to remove")]
    UnmergedChanges,

    /// Worktree is stale (diverged from reference)
    #[error("Worktree has diverged from reference branch")]
    Stale,

    /// Invalid branch name
    #[error("Invalid branch name: {0}")]
    InvalidBranchName(String),

    /// Invalid path
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Git operation failed
    #[error("Git operation failed: {0}")]
    GitError(String),

    /// IO operation failed
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for WorktreeError {
    fn from(err: std::io::Error) -> Self {
        WorktreeError::IoError(err.to_string())
    }
}

/// Result type alias for domain operations
pub type DomainResult<T> = Result<T, WorktreeError>;
