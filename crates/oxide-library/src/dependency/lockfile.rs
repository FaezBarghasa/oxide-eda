//! Lockfile manager for reading, writing, and validating `project.lock`.

use std::path::{Path, PathBuf};
use oxide_types::project::ProjectLockfile;

use super::DependencyError;

/// Manager for `project.lock` files.
#[derive(Debug, Default, Clone)]
pub struct LockfileManager;

impl LockfileManager {
    pub const LOCKFILE_NAME: &'static str = "project.lock";

    pub fn new() -> Self {
        Self
    }

    /// Path to `project.lock` for a given project directory.
    pub fn lockfile_path(project_root: &Path) -> PathBuf {
        project_root.join(Self::LOCKFILE_NAME)
    }

    /// Read `project.lock` if it exists.
    pub fn read_lockfile(project_root: &Path) -> Result<Option<ProjectLockfile>, DependencyError> {
        let path = Self::lockfile_path(project_root);
        if !path.exists() {
            return Ok(None);
        }

        let bytes = std::fs::read(&path).map_err(|source| DependencyError::Io {
            path: path.display().to_string(),
            source,
        })?;

        let lockfile: ProjectLockfile = serde_json::from_slice(&bytes).map_err(|e| {
            DependencyError::Lockfile(format!("Corrupt `{}`: {e}", path.display()))
        })?;

        Ok(Some(lockfile))
    }

    /// Write `project.lock` atomically.
    pub fn write_lockfile(
        project_root: &Path,
        lockfile: &ProjectLockfile,
    ) -> Result<(), DependencyError> {
        let path = Self::lockfile_path(project_root);
        let json = serde_json::to_vec_pretty(lockfile).map_err(|e| {
            DependencyError::Lockfile(format!("Failed to serialize lockfile: {e}"))
        })?;

        oxide_types::atomic_io::atomic_write(&path, &json).map_err(|source| {
            DependencyError::Io {
                path: path.display().to_string(),
                source,
            }
        })?;

        Ok(())
    }
}
