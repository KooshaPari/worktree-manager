//! Domain layer - pure business logic with zero external dependencies
//!
//! Following Hexagonal Architecture:
//! - Domain contains entities, value objects, and domain events
//! - No dependencies on infrastructure or application layers
//! - All business rules are encapsulated here

pub mod errors;
pub mod models;

pub use errors::{DomainResult, WorktreeError};
pub use models::{
    BranchName, CleanupPolicy, Worktree, WorktreeId, WorktreeListing, WorktreeResult,
};
