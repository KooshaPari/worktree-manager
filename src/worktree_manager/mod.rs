//! worktree-manager - Git worktree automation
//!
//! A hexagonal architecture implementation for git worktree management.

pub mod domain;
pub mod ports;
pub mod application;
pub mod infrastructure;
pub mod cli;

// Re-exports for convenience
pub use domain::*;
pub use ports::*;
pub use application::WorktreeService;
pub use infrastructure::{GitWorktreeAdapter, SimpleFilesystemAdapter};
