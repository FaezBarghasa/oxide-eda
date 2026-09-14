//! High-level Git2 dependency manager for Oxide EDA projects.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use oxide_types::project::{
    DependencyKind, LibraryEntry, LibraryEntryKind, LockedDependency, ProjectData,
    ProjectDependency, ProjectLockfile,
};

use super::{
    DependencyError, installer::GitInstaller, lockfile::LockfileManager,
    resolver::{GitResolver, ResolvedRef},
};

/// Verification status report across all dependencies.
#[derive(Debug, Clone, Default)]
pub struct VerificationReport {
    pub valid_dependencies: Vec<String>,
    pub missing_dependencies: Vec<String>,
    pub corrupted_dependencies: Vec<String>,
}

impl VerificationReport {
    pub fn is_clean(&self) -> bool {
        self.missing_dependencies.is_empty() && self.corrupted_dependencies.is_empty()
    }
}

/// Mount summary for integrating resolved dependencies into an active EDA project.
#[derive(Debug, Clone, Default)]
pub struct MountReport {
    /// Mounted `.snxlib` libraries added to `ProjectData.libraries`.
    pub mounted_libraries: Vec<PathBuf>,
    /// Search paths added for footprint packages (`.snxfpt`).
    pub footprint_search_paths: Vec<PathBuf>,
    /// Search paths added for symbol packages (`.snxsym`).
    pub component_search_paths: Vec<PathBuf>,
}

/// The Git2 dependency and version manager.
#[derive(Debug, Default, Clone)]
pub struct GitDependencyManager {
    resolver: GitResolver,
    installer: GitInstaller,
    lockfile_mgr: LockfileManager,
}

impl GitDependencyManager {
    pub fn new() -> Self {
        Self {
            resolver: GitResolver::new(),
            installer: GitInstaller::new(),
            lockfile_mgr: LockfileManager::new(),
        }
    }

    /// Resolve and update all dependencies declared in `project_data`,
    /// cloning/updating them into `.oxide/deps/` and writing `project.lock`.
    pub fn resolve_and_install(
        &self,
        project_root: &Path,
        project_data: &ProjectData,
    ) -> Result<ProjectLockfile, DependencyError> {
        let mut new_locked = BTreeMap::new();

        for dep in &project_data.dependencies {
            if !dep.enabled {
                continue;
            }

            // Probe or clone repository temporarily if needed for resolution
            let install_path = GitInstaller::default_install_path(&dep.name);
            let abs_path = project_root.join(&install_path);

            let repo = if abs_path.join(".git").exists() {
                let repo = git2::Repository::open(&abs_path)?;
                if let Ok(mut remote) = repo.find_remote("origin") {
                    let mut opts = git2::FetchOptions::new();
                    let refspecs: [&str; 0] = [];
                    let _ = remote.fetch(&refspecs, Some(&mut opts), None);
                }
                repo
            } else {
                // If the source URL is a local directory, open it directly to resolve
                let url_path = Path::new(&dep.source.url);
                if url_path.exists() && url_path.join(".git").exists() {
                    git2::Repository::open(url_path)?
                } else {
                    // Clone directly into cache
                    git2::Repository::clone(&dep.source.url, &abs_path)?
                }
            };

            let resolved = self.resolver.resolve_reference(&repo, dep)?;
            let locked_dep = self.installer.install_or_update(project_root, dep, &resolved)?;
            new_locked.insert(dep.name.clone(), locked_dep);
        }

        let lockfile = ProjectLockfile {
            version: 1,
            dependencies: new_locked,
        };

        LockfileManager::write_lockfile(project_root, &lockfile)?;
        Ok(lockfile)
    }

    /// Restore exact dependencies strictly from `project.lock`.
    pub fn restore_from_lockfile(
        &self,
        project_root: &Path,
        lockfile: &ProjectLockfile,
    ) -> Result<(), DependencyError> {
        for (name, locked) in &lockfile.dependencies {
            let dep = ProjectDependency {
                name: locked.name.clone(),
                kind: locked.kind,
                source: oxide_types::project::GitSource {
                    url: locked.url.clone(),
                    reference: oxide_types::project::GitReference::Rev(locked.commit_oid.clone()),
                },
                subpath: None,
                enabled: true,
            };

            let commit_oid = git2::Oid::from_str(&locked.commit_oid).map_err(|e| {
                DependencyError::Lockfile(format!("Invalid commit OID for `{name}`: {e}"))
            })?;
            let tree_oid = git2::Oid::from_str(&locked.tree_oid).map_err(|e| {
                DependencyError::Lockfile(format!("Invalid tree OID for `{name}`: {e}"))
            })?;

            let resolved = ResolvedRef {
                commit_oid,
                tree_oid,
                resolved_version: locked.resolved_version.clone(),
            };

            self.installer.install_or_update(project_root, &dep, &resolved)?;
        }

        Ok(())
    }

    /// Verify local disk state against `project.lock`.
    pub fn verify_integrity(
        &self,
        project_root: &Path,
        lockfile: &ProjectLockfile,
    ) -> Result<VerificationReport, DependencyError> {
        let mut report = VerificationReport::default();

        for (name, locked) in &lockfile.dependencies {
            let abs_path = project_root.join(&locked.install_path);
            if !abs_path.exists() {
                report.missing_dependencies.push(name.clone());
                continue;
            }

            match self.installer.verify_integrity(project_root, locked) {
                Ok(true) => report.valid_dependencies.push(name.clone()),
                Ok(false) => report.missing_dependencies.push(name.clone()),
                Err(_) => report.corrupted_dependencies.push(name.clone()),
            }
        }

        Ok(report)
    }

    /// Mount resolved dependencies into `ProjectData` and collect search paths.
    pub fn mount_dependencies(
        &self,
        project_root: &Path,
        lockfile: &ProjectLockfile,
        project_data: &mut ProjectData,
    ) -> Result<MountReport, DependencyError> {
        let mut report = MountReport::default();

        for (_name, locked) in &lockfile.dependencies {
            let abs_path = project_root.join(&locked.install_path);
            match locked.kind {
                DependencyKind::Library => {
                    // Check if this is a .snxlib directory or contains a .snxlib file
                    let library_entry = LibraryEntry {
                        path: locked.install_path.clone(),
                        kind: LibraryEntryKind::ProjectLocal,
                        library_id: None,
                    };

                    // Only add if not already in project_data.libraries
                    if !project_data.libraries.iter().any(|lib| lib.path == locked.install_path) {
                        project_data.libraries.push(library_entry);
                    }
                    report.mounted_libraries.push(abs_path);
                }
                DependencyKind::Footprint => {
                    report.footprint_search_paths.push(abs_path);
                }
                DependencyKind::Component => {
                    report.component_search_paths.push(abs_path);
                }
            }
        }

        Ok(report)
    }
}
