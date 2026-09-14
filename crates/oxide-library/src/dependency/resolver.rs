//! Git2 resolver for EDA project dependencies.
//!
//! Evaluates Git tags against semantic version ranges, branches, and commits.

use oxide_types::project::{GitReference, ProjectDependency};
use semver::{Version, VersionReq};

use super::DependencyError;

/// Resolved target commit and version metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRef {
    pub commit_oid: git2::Oid,
    pub tree_oid: git2::Oid,
    pub resolved_version: Option<String>,
}

/// Git resolver engine using `git2`.
#[derive(Debug, Default, Clone)]
pub struct GitResolver;

impl GitResolver {
    pub fn new() -> Self {
        Self
    }

    /// Resolve a dependency's requested Git reference to a concrete commit & tree OID.
    pub fn resolve_reference(
        &self,
        repo: &git2::Repository,
        dep: &ProjectDependency,
    ) -> Result<ResolvedRef, DependencyError> {
        match &dep.source.reference {
            GitReference::Tag(tag_name) => self.resolve_tag(repo, &dep.name, tag_name),
            GitReference::Branch(branch_name) => self.resolve_branch(repo, &dep.name, branch_name),
            GitReference::Rev(rev_str) => self.resolve_rev(repo, &dep.name, rev_str),
            GitReference::Semver(req_str) => self.resolve_semver(repo, &dep.name, req_str),
        }
    }

    /// Resolve an exact tag name.
    fn resolve_tag(
        &self,
        repo: &git2::Repository,
        name: &str,
        tag_name: &str,
    ) -> Result<ResolvedRef, DependencyError> {
        let refname = format!("refs/tags/{}", tag_name);
        let reference = repo.find_reference(&refname).or_else(|_| {
            // Fallback: try direct reference or short tag lookup
            repo.resolve_reference_from_short_name(tag_name)
        }).map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Tag `{tag_name}` not found in repository: {e}"),
        })?;

        let peeled = reference.peel_to_commit().map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Could not peel tag `{tag_name}` to commit: {e}"),
        })?;

        let tree = peeled.tree().map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Could not read tree for tag `{tag_name}`: {e}"),
        })?;

        let clean_version = tag_name.strip_prefix('v').or_else(|| tag_name.strip_prefix('V')).unwrap_or(tag_name);

        Ok(ResolvedRef {
            commit_oid: peeled.id(),
            tree_oid: tree.id(),
            resolved_version: Some(clean_version.to_string()),
        })
    }

    /// Resolve a branch head.
    fn resolve_branch(
        &self,
        repo: &git2::Repository,
        name: &str,
        branch_name: &str,
    ) -> Result<ResolvedRef, DependencyError> {
        let branch = repo.find_branch(branch_name, git2::BranchType::Local)
            .or_else(|_| repo.find_branch(&format!("origin/{branch_name}"), git2::BranchType::Remote))
            .or_else(|_| repo.find_branch(branch_name, git2::BranchType::Remote))
            .map_err(|e| DependencyError::Resolution {
                name: name.to_string(),
                reason: format!("Branch `{branch_name}` not found: {e}"),
            })?;

        let reference = branch.into_reference();
        let commit = reference.peel_to_commit().map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Could not peel branch `{branch_name}` to commit: {e}"),
        })?;

        let tree = commit.tree().map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Could not read tree for branch `{branch_name}`: {e}"),
        })?;

        Ok(ResolvedRef {
            commit_oid: commit.id(),
            tree_oid: tree.id(),
            resolved_version: None,
        })
    }

    /// Resolve a direct commit hash / revision string.
    fn resolve_rev(
        &self,
        repo: &git2::Repository,
        name: &str,
        rev_str: &str,
    ) -> Result<ResolvedRef, DependencyError> {
        let obj = repo.revparse_single(rev_str).map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Revision `{rev_str}` could not be parsed: {e}"),
        })?;

        let commit = obj.peel_to_commit().map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Revision `{rev_str}` is not a commit: {e}"),
        })?;

        let tree = commit.tree().map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Could not read tree for rev `{rev_str}`: {e}"),
        })?;

        Ok(ResolvedRef {
            commit_oid: commit.id(),
            tree_oid: tree.id(),
            resolved_version: None,
        })
    }

    /// Resolve semantic version requirements against available Git tags.
    fn resolve_semver(
        &self,
        repo: &git2::Repository,
        name: &str,
        req_str: &str,
    ) -> Result<ResolvedRef, DependencyError> {
        let version_req = VersionReq::parse(req_str).map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Invalid semver requirement `{req_str}`: {e}"),
        })?;

        let tag_names = repo.tag_names(None).map_err(|e| DependencyError::Resolution {
            name: name.to_string(),
            reason: format!("Failed to read tag names: {e}"),
        })?;

        let mut matched_versions: Vec<(Version, String)> = Vec::new();

        for tag_res in tag_names.iter() {
            if let Some(tag_name) = tag_res {
                let clean = tag_name.strip_prefix('v').or_else(|| tag_name.strip_prefix('V')).unwrap_or(tag_name);
                if let Ok(ver) = Version::parse(clean) {
                    if version_req.matches(&ver) {
                        matched_versions.push((ver, tag_name.to_string()));
                    }
                }
            }
        }

        if matched_versions.is_empty() {
            return Err(DependencyError::Resolution {
                name: name.to_string(),
                reason: format!("No Git tags match semver constraint `{req_str}`"),
            });
        }

        // Sort ascending, pick highest match
        matched_versions.sort_by(|a, b| a.0.cmp(&b.0));
        let (best_ver, best_tag) = matched_versions.pop().expect("checked not empty");

        let mut resolved = self.resolve_tag(repo, name, &best_tag)?;
        resolved.resolved_version = Some(best_ver.to_string());
        Ok(resolved)
    }
}
