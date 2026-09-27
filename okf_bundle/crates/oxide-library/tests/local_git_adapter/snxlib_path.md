---
okf_version: "0.2"
type: Function
title: snxlib_path
description: "Per-library .snxlib path inside `dir`. The file lives one level"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/snxlib_path
language: rust
---

# snxlib_path

Per-library .snxlib path inside `dir`. The file lives one level

## Signature

```rust
fn snxlib_path(dir: &Path, name: &str) -> PathBuf
```

## Docstring

Per-library .snxlib path inside `dir`. The file lives one level
down (`dir/<name>/<name>.snxlib`) so each test gets its own
directory for the parent-keyed git repo.

## Source
Lines 51–53 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| called_by | [cascade_footprint_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_footprint_personal_mode_auto_bumps_bound_row.md) |
| called_by | [cascade_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_personal_mode_auto_bumps_bound_row.md) |
| called_by | [cascade_team_mode_leaves_released_row_stale](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_leaves_released_row_stale.md) |
| called_by | [cascade_team_mode_unreleased_row_auto_bumps](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_unreleased_row_auto_bumps.md) |
| called_by | [corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid](/crates/oxide-library/tests/local_git_adapter/corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid.md) |
| called_by | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
| called_by | [init_open_round_trip_empty_library](/crates/oxide-library/tests/local_git_adapter/init_open_round_trip_empty_library.md) |
| called_by | [init_refuses_existing_library](/crates/oxide-library/tests/local_git_adapter/init_refuses_existing_library.md) |
| called_by | [library_id_returns_manifest_id](/crates/oxide-library/tests/local_git_adapter/library_id_returns_manifest_id.md) |
| called_by | [library_set_resolves_across_two_local_libs](/crates/oxide-library/tests/local_git_adapter/library_set_resolves_across_two_local_libs.md) |
| called_by | [local_git_commits_with_message](/crates/oxide-library/tests/local_git_adapter/local_git_commits_with_message.md) |
| called_by | [local_git_iter_rows_across_tables](/crates/oxide-library/tests/local_git_adapter/local_git_iter_rows_across_tables.md) |
| called_by | [local_git_read_row_by_pn](/crates/oxide-library/tests/local_git_adapter/local_git_read_row_by_pn.md) |
| called_by | [local_git_round_trip_row](/crates/oxide-library/tests/local_git_adapter/local_git_round_trip_row.md) |
