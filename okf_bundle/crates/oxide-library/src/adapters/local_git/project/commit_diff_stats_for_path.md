---
okf_version: "0.2"
type: Function
title: commit_diff_stats_for_path
description: "Returns `Some(stats)` when `commit` touched `rel_str`; `None` when"
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/commit_diff_stats_for_path
language: rust
---

# commit_diff_stats_for_path

Returns `Some(stats)` when `commit` touched `rel_str`; `None` when

## Signature

```rust
fn commit_diff_stats_for_path(
    repo: &git2::Repository,
    commit: &git2::Commit<'_>,
    rel_str: &str,
) -> Result<Option<CommitPathStats>, LibraryError>
```

## Docstring

Returns `Some(stats)` when `commit` touched `rel_str`; `None` when
it didn't. Replaces the v0.22 `commit_touches_path` boolean
version — populating stats here avoids a second tree-walk per row.

## Source
Lines 381–429 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
| called_by | [file_history](/crates/oxide-library/src/adapters/local_git/project/file_history.md) |
