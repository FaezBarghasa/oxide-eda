---
okf_version: "0.2"
type: Function
title: write_file
resource: crates/oxide-library/tests/local_git_project_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/local_git_project_adapter/write_file
language: rust
---

# write_file

## Signature

```rust
fn write_file(root: &Path, rel: &str, content: &str)
```

## Source
Lines 20–26 in `crates/oxide-library/tests/local_git_project_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_project_adapter](/crates/oxide-library/tests/local_git_project_adapter.md) |
| called_by | [commit_external_change_creates_a_user_edit_commit](/crates/oxide-library/tests/local_git_project_adapter/commit_external_change_creates_a_user_edit_commit.md) |
| called_by | [commit_path_chains_subsequent_commits](/crates/oxide-library/tests/local_git_project_adapter/commit_path_chains_subsequent_commits.md) |
| called_by | [commit_path_creates_first_commit_on_unborn_head](/crates/oxide-library/tests/local_git_project_adapter/commit_path_creates_first_commit_on_unborn_head.md) |
| called_by | [commit_path_populates_history_diff_stats](/crates/oxide-library/tests/local_git_project_adapter/commit_path_populates_history_diff_stats.md) |
| called_by | [file_history_caps_walker_iterations_for_dos_protection](/crates/oxide-library/tests/local_git_project_adapter/file_history_caps_walker_iterations_for_dos_protection.md) |
| called_by | [file_history_filters_by_path](/crates/oxide-library/tests/local_git_project_adapter/file_history_filters_by_path.md) |
| called_by | [file_history_respects_limit](/crates/oxide-library/tests/local_git_project_adapter/file_history_respects_limit.md) |
| called_by | [restore_at_from_sha_round_trips_via_string_oid](/crates/oxide-library/tests/local_git_project_adapter/restore_at_from_sha_round_trips_via_string_oid.md) |
| called_by | [restore_at_round_trips_a_prior_version](/crates/oxide-library/tests/local_git_project_adapter/restore_at_round_trips_a_prior_version.md) |
