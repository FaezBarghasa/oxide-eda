---
okf_version: "0.2"
type: Function
title: commit_path
description: "Commit a single file with the supplied message. `rel_path` is"
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/commit_path_1
language: rust
---

# commit_path

Commit a single file with the supplied message. `rel_path` is

## Signature

```rust
pub fn commit_path(&self, rel_path: &Path, message: &str) -> Result<git2::Oid, LibraryError>
```

## Visibility

- `pub`

## Docstring

Commit a single file with the supplied message. `rel_path` is
relative to the project root and uses forward slashes.
Returns the commit OID on success.

Uses an unborn-HEAD-tolerant parent-commit lookup so the very
first commit on a fresh `git init` succeeds without the
caller needing to handle that edge case.

## Source
Lines 97–159 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
| calls | [identity_for_repo](/crates/oxide-library/src/adapters/local_git/project/identity_for_repo.md) |
| calls | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
