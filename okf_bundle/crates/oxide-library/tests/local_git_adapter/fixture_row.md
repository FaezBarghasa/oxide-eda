---
okf_version: "0.2"
type: Function
title: fixture_row
description: "Build a fixture row with a given internal PN and class. The `lib_id`"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/fixture_row
language: rust
---

# fixture_row

Build a fixture row with a given internal PN and class. The `lib_id`

## Signature

```rust
fn fixture_row(internal_pn: &str, class: &str, lib_id: Uuid) -> ComponentRow
```

## Docstring

Build a fixture row with a given internal PN and class. The `lib_id`
argument is the library this row's primitives live in — the adapter's
own `library_id` for an in-library row, or some other lib for a row
pointing at an external primitive.

## Source
Lines 703–729 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| called_by | [cascade_footprint_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_footprint_personal_mode_auto_bumps_bound_row.md) |
| called_by | [cascade_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_personal_mode_auto_bumps_bound_row.md) |
| called_by | [cascade_team_mode_leaves_released_row_stale](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_leaves_released_row_stale.md) |
| called_by | [cascade_team_mode_unreleased_row_auto_bumps](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_unreleased_row_auto_bumps.md) |
| called_by | [local_git_commits_with_message](/crates/oxide-library/tests/local_git_adapter/local_git_commits_with_message.md) |
| called_by | [local_git_iter_rows_across_tables](/crates/oxide-library/tests/local_git_adapter/local_git_iter_rows_across_tables.md) |
| called_by | [local_git_read_row_by_pn](/crates/oxide-library/tests/local_git_adapter/local_git_read_row_by_pn.md) |
| called_by | [local_git_round_trip_row](/crates/oxide-library/tests/local_git_adapter/local_git_round_trip_row.md) |
