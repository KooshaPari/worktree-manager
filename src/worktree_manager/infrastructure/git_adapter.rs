//! Git adapter using subprocess commands
//!
//! Following Hexagonal Architecture: Infrastructure (Driven Adapter).

use crate::domain::{BranchName, WorktreeError, DomainResult, Worktree, WorktreeId, WorktreeListing};
use crate::ports::{BranchOperations, WorktreeRepository};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Git worktree adapter using git commands
#[derive(Clone)]
pub struct GitWorktreeAdapter;

impl GitWorktreeAdapter {
    pub fn new() -> Self {
        Self
    }

    fn run_git(&self, repo_path: &Path, args: &[&str]) -> Result<String, WorktreeError> {
        let output = Command::new("git")
            .args(["-C", repo_path.to_str().unwrap_or(".")])
            .args(args)
            .output()
            .map_err(|e| WorktreeError::GitError(format!("git command failed: {}", e)))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(WorktreeError::GitError(format!("git failed: {}", stderr)))
        }
    }
}

impl Default for GitWorktreeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl WorktreeRepository for GitWorktreeAdapter {
    fn list(&self, repo_path: &Path) -> DomainResult<WorktreeListing> {
        let output = self.run_git(repo_path, &["worktree", "list", "--porcelain"])?;
        
        let mut worktrees: Vec<Worktree> = Vec::new();
        let mut main: Option<Worktree> = None;
        let mut current: Option<Worktree> = None;
        
        for line in output.lines() {
            if line.starts_with("worktree ") {
                if let Some(wt) = current.take() {
                    if wt.is_main {
                        main = Some(wt);
                    } else {
                        worktrees.push(wt);
                    }
                }
                
                let path = line.trim_start_matches("worktree ");
                let is_main = path.contains("/.git/worktrees") || path == repo_path.to_str().unwrap_or("");
                current = Some(Worktree {
                    id: WorktreeId(PathBuf::from(path)),
                    branch: BranchName::default(),
                    path: PathBuf::from(path),
                    head: String::new(),
                    created_at: chrono::Utc::now(),
                    is_main,
                    locked: false,
                    lock_reason: None,
                });
            } else if let Some(ref mut wt) = current {
                if line.starts_with("branch ") {
                    wt.branch = BranchName::new(line.trim_start_matches("branch ").trim());
                } else if line.starts_with("head ") {
                    wt.head = line.trim_start_matches("head ").to_string();
                } else if line.starts_with("locked ") {
                    wt.locked = true;
                    wt.lock_reason = Some(line.trim_start_matches("locked ").to_string());
                }
            }
        }
        
        if let Some(wt) = current {
            if wt.is_main {
                main = Some(wt);
            } else {
                worktrees.push(wt);
            }
        }
        
        let main = main.unwrap_or_else(|| Worktree::main(repo_path.to_path_buf(), String::new()));
        let total_count = worktrees.len();
        
        Ok(WorktreeListing {
            worktrees: worktrees.clone(),
            main,
            total_count,
        })
    }

    fn create(&self, repo_path: &Path, branch: &BranchName, worktree_path: &Path) -> DomainResult<Worktree> {
        let path_str = worktree_path.to_str().ok_or_else(|| {
            WorktreeError::InvalidPath("Invalid worktree path".to_string())
        })?;
        
        let _output = self.run_git(repo_path, &["worktree", "add", "-b", branch.as_str(), path_str, "HEAD"])?;
        
        Ok(Worktree::new(branch.clone(), worktree_path.to_path_buf(), "HEAD".to_string()))
    }

    fn remove(&self, worktree_path: &Path, force: bool) -> DomainResult<()> {
        let path_str = worktree_path.to_str().ok_or_else(|| {
            WorktreeError::InvalidPath("Invalid worktree path".to_string())
        })?;
        
        let mut args = vec!["worktree", "remove", path_str];
        if force {
            args.push("--force");
        }
        
        self.run_git(worktree_path, &args)?;
        Ok(())
    }

    fn lock(&self, worktree_path: &Path, reason: &str) -> DomainResult<()> {
        let path_str = worktree_path.to_str().ok_or_else(|| {
            WorktreeError::InvalidPath("Invalid worktree path".to_string())
        })?;
        
        self.run_git(worktree_path, &["worktree", "lock", path_str, "--reason", reason])?;
        Ok(())
    }

    fn unlock(&self, worktree_path: &Path) -> DomainResult<()> {
        let path_str = worktree_path.to_str().ok_or_else(|| {
            WorktreeError::InvalidPath("Invalid worktree path".to_string())
        })?;
        
        self.run_git(worktree_path, &["worktree", "unlock", path_str])?;
        Ok(())
    }

    fn prune(&self, repo_path: &Path) -> DomainResult<()> {
        self.run_git(repo_path, &["worktree", "prune"])?;
        Ok(())
    }
}

/// Simple filesystem adapter for lock files
#[derive(Clone)]
pub struct SimpleFilesystemAdapter;

impl SimpleFilesystemAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SimpleFilesystemAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl BranchOperations for GitWorktreeAdapter {
    fn exists(&self, repo_path: &Path, branch: &BranchName) -> DomainResult<bool> {
        let output = self.run_git(repo_path, &["rev-parse", "--verify", &format!("origin/{}", branch.as_str())])?;
        Ok(!output.trim().is_empty())
    }

    fn create(&self, repo_path: &Path, branch: &BranchName, from_ref: Option<&str>) -> DomainResult<()> {
        let mut args = vec!["checkout", "-b", branch.as_str()];
        if let Some(ref_name) = from_ref {
            args.push(ref_name);
        }
        
        self.run_git(repo_path, &args)?;
        Ok(())
    }

    fn delete(&self, repo_path: &Path, branch: &BranchName) -> DomainResult<()> {
        self.run_git(repo_path, &["branch", "-d", branch.as_str()])?;
        Ok(())
    }

    fn current(&self, repo_path: &Path) -> DomainResult<BranchName> {
        let output = self.run_git(repo_path, &["rev-parse", "--abbrev-ref", "HEAD"])?;
        Ok(BranchName::new(output.trim()))
    }
}
