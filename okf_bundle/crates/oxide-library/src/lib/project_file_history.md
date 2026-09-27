---
okf_version: "0.2"
type: Function
title: project_file_history
description: "Walk the commit graph at `project_dir` and return up to 50"
resource: crates/oxide-library/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:59:15Z"
concept_id: crates/oxide-library/src/lib/project_file_history
language: rust
---

# project_file_history

Walk the commit graph at `project_dir` and return up to 50

## Signature

```rust
pub fn project_file_history(
    project_dir: &std::path::Path,
    rel_path: &std::path::Path,
) -> Result<Vec<adapter::HistoryEntry>, adapter::LibraryError>
```

## Decorators

- `cfg(feature = "local-git")`

## Visibility

- `pub`

## Docstring

Walk the commit graph at `project_dir` and return up to 50
commits that touched `rel_path` (relative to `project_dir`).

Mirrors [`crate::adapters::local_git::LocalGitAdapter::history`]
but works on **any** git repository — not just library-rooted
ones. Used by `oxide-app`'s right-dock History panel to show
the active tab's file history regardless of whether the file
lives inside a `.snxlib` or in a plain Oxide project.

Returns `Ok(vec![])` when the path has no commits yet (fresh
repo, untracked file, unborn HEAD). Returns
`Err(LibraryError::NotFound)` when `project_dir` has no `.git/`.
[cfg(feature = "local-git")]

## Source
Lines 253–397 in `crates/oxide-library/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-library/src/lib.md) |
| calls | [commit_touches_path](/crates/oxide-library/src/lib/commit_touches_path.md) |
| calls | [commit_to_history_entry](/crates/oxide-library/src/lib/commit_to_history_entry.md) |
| called_by | [refresh_history_panel](/crates/oxide-app/src/app/runtime/history/refresh_history_panel.md) |
| called_by | [accepts_absolute_path_under_project_dir](/crates/oxide-library/tests/project_file_history/accepts_absolute_path_under_project_dir.md) |
| called_by | [returns_empty_for_path_with_no_commits](/crates/oxide-library/tests/project_file_history/returns_empty_for_path_with_no_commits.md) |
| called_by | [returns_empty_on_unborn_head](/crates/oxide-library/tests/project_file_history/returns_empty_on_unborn_head.md) |
| called_by | [returns_n_commits_newest_first](/crates/oxide-library/tests/project_file_history/returns_n_commits_newest_first.md) |
| called_by | [returns_not_found_when_no_dot_git](/crates/oxide-library/tests/project_file_history/returns_not_found_when_no_dot_git.md) |
