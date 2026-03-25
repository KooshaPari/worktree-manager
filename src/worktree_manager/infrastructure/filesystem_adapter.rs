//! Filesystem adapter implementation

use crate::domain::DomainResult;
use crate::ports::FilesystemOperations;
use std::fs;
use std::path::Path;

/// Simple filesystem operations adapter
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

impl FilesystemOperations for SimpleFilesystemAdapter {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir(&self, path: &Path) -> DomainResult<()> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    fn remove_dir(&self, path: &Path) -> DomainResult<()> {
        fs::remove_dir_all(path)?;
        Ok(())
    }

    fn is_empty(&self, path: &Path) -> DomainResult<bool> {
        if !path.exists() {
            return Ok(true);
        }
        
        let entries = fs::read_dir(path)?;
        Ok(entries.count() == 0)
    }
}
