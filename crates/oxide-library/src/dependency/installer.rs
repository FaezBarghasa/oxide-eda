//! Git2 installer for cloning, fetching, checking out, and verifying EDA dependencies.

use std::path::{Path, PathBuf};
use oxide_types::project::{LockedDependency, ProjectDependency};

use super::{DependencyError, resolver::ResolvedRef};

/// Installer responsible for on-disk repository caches in `.oxide/deps/`.
#[derive(Debug, Default, Clone)]
pub struct GitInstaller;

impl GitInstaller {
    pub fn new() -> Self {
        Self
    }

    /// Compute default install path for a dependency relative to the project root.
    pub fn default_install_path(name: &str) -> PathBuf {
        PathBuf::from(".oxide").join("deps").join(name)
    }

    /// Clone or open repository at destination, fetch remotes, and checkout the exact commit.
    pub fn install_or_update(
        &self,
        project_root: &Path,
        dep: &ProjectDependency,
        resolved: &ResolvedRef,
    ) -> Result<LockedDependency, DependencyError> {
        let rel_install_path = Self::default_install_path(&dep.name);
        let abs_install_path = project_root.join(&rel_install_path);

        if !abs_install_path.exists() {
            std::fs::create_dir_all(&abs_install_path).map_err(|source| DependencyError::Io {
                path: abs_install_path.display().to_string(),
                source,
            })?;
        }

        let repo = if abs_install_path.join(".git").exists() {
            let repo = git2::Repository::open(&abs_install_path)?;
            // If it's a remote URL and not a local path clone, fetch latest
            if let Ok(mut remote) = repo.find_remote("origin") {
                let mut fetch_opts = git2::FetchOptions::new();
                let refspecs: [&str; 0] = [];
                let _ = remote.fetch(&refspecs, Some(&mut fetch_opts), None);
            }
            repo
        } else {
            // Clone the repository
            git2::Repository::clone(&dep.source.url, &abs_install_path)?
        };

        // Checkout the exact commit in detached HEAD mode
        let commit = repo.find_commit(resolved.commit_oid)?;
        let tree = commit.tree()?;

        let mut checkout_builder = git2::build::CheckoutBuilder::new();
        checkout_builder.force();

        repo.set_head_detached(resolved.commit_oid)?;
        repo.checkout_head(Some(&mut checkout_builder))?;

        let locked = LockedDependency {
            name: dep.name.clone(),
            kind: dep.kind,
            url: dep.source.url.clone(),
            commit_oid: resolved.commit_oid.to_string(),
            tree_oid: tree.id().to_string(),
            resolved_version: resolved.resolved_version.clone(),
            install_path: rel_install_path,
            locked_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok(locked)
    }

    /// Verify that an installed dependency on disk matches the exact tree OID in the lockfile.
    pub fn verify_integrity(
        &self,
        project_root: &Path,
        locked: &LockedDependency,
    ) -> Result<bool, DependencyError> {
        let abs_install_path = project_root.join(&locked.install_path);
        if !abs_install_path.exists() {
            return Ok(false);
        }

        let repo = git2::Repository::open(&abs_install_path)?;
        let head = match repo.head() {
            Ok(h) => h,
            Err(_) => return Ok(false),
        };

        let commit = head.peel_to_commit()?;
        let tree = commit.tree()?;

        let actual_commit_oid = commit.id().to_string();
        let actual_tree_oid = tree.id().to_string();

        if actual_commit_oid != locked.commit_oid || actual_tree_oid != locked.tree_oid {
            return Err(DependencyError::Integrity {
                name: locked.name.clone(),
                expected: locked.tree_oid.clone(),
                actual: actual_tree_oid,
            });
        }

        Ok(true)
    }
}
