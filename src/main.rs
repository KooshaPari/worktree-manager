//! worktree-manager CLI entry point

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use worktree_manager::{
    cli::{Cli, Commands},
    application::WorktreeService,
    domain::BranchName,
    infrastructure::GitWorktreeAdapter,
};

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "wtm=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    // Determine repo path
    let repo_path = cli.repo
        .map(Ok)
        .unwrap_or_else(std::env::current_dir)?;

    // Create adapter and service
    let adapter = GitWorktreeAdapter::new();
    let service = WorktreeService::new(adapter.clone(), adapter);

    match cli.command {
        Commands::List { json } => {
            let listing = service.list_worktrees(&repo_path)?;
            
            if json {
                println!("{}", serde_json::to_string_pretty(&listing)?);
            } else {
                println!("Main: {}", listing.main.path.display());
                println!("Worktrees ({}):", listing.worktrees.len());
                for wt in &listing.worktrees {
                    let status = if wt.locked { "🔒" } else { "   " };
                    println!("  {} {} -> {}", status, wt.branch.as_str(), wt.path.display());
                }
            }
        }
        
        Commands::Create { branch, path, start_point } => {
            let branch_name = BranchName::new(branch);
            let result = service.create_worktree(&repo_path, branch_name, &path, start_point.as_deref());
            
            if result.success {
                println!("Created worktree: {}", result.worktree.unwrap().path.display());
                for warning in result.warnings {
                    eprintln!("Warning: {}", warning);
                }
            } else {
                eprintln!("Failed: {}", result.error.unwrap());
                std::process::exit(1);
            }
        }
        
        Commands::Remove { path, force } => {
            service.remove_worktree(&path, force)?;
            println!("Removed worktree: {}", path.display());
        }
        
        Commands::Lock { path, reason } => {
            service.lock_worktree(&path, &reason)?;
            println!("Locked: {}", path.display());
        }
        
        Commands::Unlock { path } => {
            service.unlock_worktree(&path)?;
            println!("Unlocked: {}", path.display());
        }
        
        Commands::Prune => {
            service.prune(&repo_path)?;
            println!("Pruned stale worktree references");
        }
        
        Commands::Branch => {
            let branch = service.current_branch(&repo_path)?;
            println!("{}", branch.as_str());
        }
        
        // Cleanup commands - simplified, just prune for now
        Commands::Cleanup { dry_run, .. } => {
            if dry_run {
                println!("Dry run - no changes made");
            } else {
                service.prune(&repo_path)?;
                println!("Pruned stale worktree references");
            }
        }
    }

    Ok(())
}
