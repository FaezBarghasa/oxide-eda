---
okf_version: "0.2"
type: Function
title: init_repo
resource: crates/oxide-library/tests/project_file_history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/project_file_history/init_repo
language: rust
---

# init_repo

## Signature

```rust
fn init_repo(dir: &Path) -> git2::Repository
```

## Source
Lines 55–58 in `crates/oxide-library/tests/project_file_history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_file_history](/crates/oxide-library/tests/project_file_history.md) |
| called_by | [accepts_absolute_path_under_project_dir](/crates/oxide-library/tests/project_file_history/accepts_absolute_path_under_project_dir.md) |
| called_by | [returns_empty_for_path_with_no_commits](/crates/oxide-library/tests/project_file_history/returns_empty_for_path_with_no_commits.md) |
| called_by | [returns_empty_on_unborn_head](/crates/oxide-library/tests/project_file_history/returns_empty_on_unborn_head.md) |
| called_by | [returns_n_commits_newest_first](/crates/oxide-library/tests/project_file_history/returns_n_commits_newest_first.md) |
