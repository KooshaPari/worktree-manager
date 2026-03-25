//! CLI module for worktree-manager

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "wtm")]
#[command(about = "Git worktree automation and management", long_about = None)]
pub struct Cli {
    /// Repository path (defaults to current directory)
    #[arg(short, long)]
    pub repo: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all worktrees
    List {
        /// Show JSON output
        #[arg(long)]
        json: bool,
    },
    
    /// Create a new worktree
    Create {
        /// Branch name
        #[arg(short, long)]
        branch: String,
        
        /// Worktree path
        #[arg(short, long)]
        path: PathBuf,
        
        /// Starting point (commit/branch)
        #[arg(short, long)]
        start_point: Option<String>,
    },
    
    /// Remove a worktree
    Remove {
        /// Worktree path
        path: PathBuf,
        
        /// Force removal
        #[arg(short, long)]
        force: bool,
    },
    
    /// Lock a worktree
    Lock {
        /// Worktree path
        path: PathBuf,
        
        /// Lock reason
        #[arg(short, long)]
        reason: String,
    },
    
    /// Unlock a worktree
    Unlock {
        /// Worktree path
        path: PathBuf,
    },
    
    /// Clean up stale worktrees
    Cleanup {
        /// Remove unmerged changes
        #[arg(long)]
        remove_unmerged: bool,
        
        /// Remove stale worktrees
        #[arg(long)]
        remove_stale: bool,
        
        /// Remove worktrees on deleted branches
        #[arg(long)]
        remove_deleted: bool,
        
        /// Dry run (don't actually remove)
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Prune worktree references
    Prune,
    
    /// Show current branch
    Branch,
}
