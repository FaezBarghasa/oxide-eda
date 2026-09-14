//! Git2-based version and dependency manager for Oxide EDA projects.
//!
//! Manages versioned dependencies (`.snxlib` libraries, `.snxfpt` footprints,
//! and `.snxsym` components) declared in `.snxprj` with deterministic locking in `project.lock`.

pub mod installer;
pub mod lockfile;
pub mod manager;
pub mod resolver;

pub use installer::GitInstaller;
pub use lockfile::LockfileManager;
pub use manager::{GitDependencyManager, MountReport, VerificationReport};
pub use resolver::GitResolver;

use thiserror::Error;

/// Errors arising during dependency resolution, fetching, locking, or mounting.
#[derive(Debug, Error)]
pub enum DependencyError {
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("I/O error at {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Lockfile error: {0}")]
    Lockfile(String),

    #[error("Version resolution error for `{name}`: {reason}")]
    Resolution { name: String, reason: String },

    #[error("Integrity check failed for `{name}`: expected tree {expected}, found {actual}")]
    Integrity {
        name: String,
        expected: String,
        actual: String,
    },

    #[error("Dependency `{name}` not found in lockfile")]
    NotFound { name: String },

    #[error("Generic backend error: {0}")]
    Backend(String),
}
