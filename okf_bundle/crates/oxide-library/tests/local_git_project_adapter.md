---
okf_version: "0.2"
type: Module
title: local_git_project_adapter
description: "v0.22 Phase 8.1 — `LocalGitProjectAdapter` integration tests."
resource: crates/oxide-library/tests/local_git_project_adapter.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/local_git_project_adapter
language: rust
---

# local_git_project_adapter

v0.22 Phase 8.1 — `LocalGitProjectAdapter` integration tests.

## Docstring

v0.22 Phase 8.1 — `LocalGitProjectAdapter` integration tests.

Mirrors `local_git_adapter.rs`'s 9-scenario coverage at the
project-scope level. Each test creates a tempdir representing a
project root (containing a fake `.snxprj` + sibling `.snxsch` /
`.snxpcb`) and walks the adapter through realistic save +
commit + history + restore flows.

These exercise the public API only — internal helpers like
`commit_touches_path` are tested indirectly via `file_history`.

## Relationships

| Type | Target |
|------|--------|
| related | [write_file](/crates/oxide-library/tests/local_git_project_adapter/write_file.md) |
| related | [read_file](/crates/oxide-library/tests/local_git_project_adapter/read_file.md) |
| related | [open_or_init_creates_dot_git_directory](/crates/oxide-library/tests/local_git_project_adapter/open_or_init_creates_dot_git_directory.md) |
| related | [open_or_init_is_idempotent_on_existing_repo](/crates/oxide-library/tests/local_git_project_adapter/open_or_init_is_idempotent_on_existing_repo.md) |
| related | [open_or_init_fails_when_root_is_not_a_directory](/crates/oxide-library/tests/local_git_project_adapter/open_or_init_fails_when_root_is_not_a_directory.md) |
| related | [commit_path_creates_first_commit_on_unborn_head](/crates/oxide-library/tests/local_git_project_adapter/commit_path_creates_first_commit_on_unborn_head.md) |
| related | [commit_path_chains_subsequent_commits](/crates/oxide-library/tests/local_git_project_adapter/commit_path_chains_subsequent_commits.md) |
| related | [file_history_filters_by_path](/crates/oxide-library/tests/local_git_project_adapter/file_history_filters_by_path.md) |
| related | [file_history_respects_limit](/crates/oxide-library/tests/local_git_project_adapter/file_history_respects_limit.md) |
| related | [file_history_empty_on_unborn_head](/crates/oxide-library/tests/local_git_project_adapter/file_history_empty_on_unborn_head.md) |
| related | [restore_at_round_trips_a_prior_version](/crates/oxide-library/tests/local_git_project_adapter/restore_at_round_trips_a_prior_version.md) |
| related | [write_gitattributes_writes_the_v022_spec_with_lfs_off](/crates/oxide-library/tests/local_git_project_adapter/write_gitattributes_writes_the_v022_spec_with_lfs_off.md) |
| related | [write_gitattributes_includes_lfs_filter_when_use_lfs_is_on](/crates/oxide-library/tests/local_git_project_adapter/write_gitattributes_includes_lfs_filter_when_use_lfs_is_on.md) |
| related | [restore_at_from_sha_round_trips_via_string_oid](/crates/oxide-library/tests/local_git_project_adapter/restore_at_from_sha_round_trips_via_string_oid.md) |
| related | [restore_at_from_sha_rejects_invalid_sha](/crates/oxide-library/tests/local_git_project_adapter/restore_at_from_sha_rejects_invalid_sha.md) |
| related | [write_gitattributes_overwrites_existing_file](/crates/oxide-library/tests/local_git_project_adapter/write_gitattributes_overwrites_existing_file.md) |
| related | [commit_external_change_creates_a_user_edit_commit](/crates/oxide-library/tests/local_git_project_adapter/commit_external_change_creates_a_user_edit_commit.md) |
| related | [commit_path_rejects_absolute_rel_path](/crates/oxide-library/tests/local_git_project_adapter/commit_path_rejects_absolute_rel_path.md) |
| related | [commit_path_rejects_parent_dir_traversal](/crates/oxide-library/tests/local_git_project_adapter/commit_path_rejects_parent_dir_traversal.md) |
| related | [commit_path_populates_history_diff_stats](/crates/oxide-library/tests/local_git_project_adapter/commit_path_populates_history_diff_stats.md) |
| related | [file_history_caps_walker_iterations_for_dos_protection](/crates/oxide-library/tests/local_git_project_adapter/file_history_caps_walker_iterations_for_dos_protection.md) |
