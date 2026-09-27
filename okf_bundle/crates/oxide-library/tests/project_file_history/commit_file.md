---
okf_version: "0.2"
type: Function
title: commit_file
description: "Stage `rel_path` (under `repo`'s working tree) and create a"
resource: crates/oxide-library/tests/project_file_history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/project_file_history/commit_file
language: rust
---

# commit_file

Stage `rel_path` (under `repo`'s working tree) and create a

## Signature

```rust
fn commit_file(
    repo: &git2::Repository,
    rel_path: &Path,
    contents: &str,
    message: &str,
) -> git2::Oid
```

## Docstring

Stage `rel_path` (under `repo`'s working tree) and create a
commit with `message`. Returns the new commit's OID for use in
follow-up assertions.

## Source
Lines 26–53 in `crates/oxide-library/tests/project_file_history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_file_history](/crates/oxide-library/tests/project_file_history.md) |
| calls | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
| calls | [fixture_signature](/crates/oxide-library/tests/project_file_history/fixture_signature.md) |
| called_by | [accepts_absolute_path_under_project_dir](/crates/oxide-library/tests/project_file_history/accepts_absolute_path_under_project_dir.md) |
| called_by | [returns_empty_for_path_with_no_commits](/crates/oxide-library/tests/project_file_history/returns_empty_for_path_with_no_commits.md) |
| called_by | [returns_n_commits_newest_first](/crates/oxide-library/tests/project_file_history/returns_n_commits_newest_first.md) |
