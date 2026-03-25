//! Infrastructure layer - adapters implementing ports
//!
//! Following Hexagonal Architecture:
//! - Contains concrete implementations of port interfaces
//! - Depends on external libraries (git2, std)
//! - Pluggable and replaceable

pub mod git_adapter;
pub mod filesystem_adapter;

pub use git_adapter::GitWorktreeAdapter;
pub use filesystem_adapter::SimpleFilesystemAdapter;
