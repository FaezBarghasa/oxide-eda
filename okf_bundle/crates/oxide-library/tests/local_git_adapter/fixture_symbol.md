---
okf_version: "0.2"
type: Function
title: fixture_symbol
description: ── Primitive CRUD ───────────────────────────────────────────────────────
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/fixture_symbol
language: rust
---

# fixture_symbol

── Primitive CRUD ───────────────────────────────────────────────────────

## Signature

```rust
fn fixture_symbol(name: &str) -> Symbol
```

## Docstring

── Primitive CRUD ───────────────────────────────────────────────────────

## Source
Lines 128–135 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| called_by | [cascade_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_personal_mode_auto_bumps_bound_row.md) |
| called_by | [cascade_team_mode_leaves_released_row_stale](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_leaves_released_row_stale.md) |
| called_by | [cascade_team_mode_unreleased_row_auto_bumps](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_unreleased_row_auto_bumps.md) |
| called_by | [corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid](/crates/oxide-library/tests/local_git_adapter/corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid.md) |
| called_by | [history_returns_per_primitive_commits](/crates/oxide-library/tests/local_git_adapter/history_returns_per_primitive_commits.md) |
| called_by | [library_set_resolves_across_two_local_libs](/crates/oxide-library/tests/local_git_adapter/library_set_resolves_across_two_local_libs.md) |
| called_by | [list_primitives_returns_alphabetic_summaries](/crates/oxide-library/tests/local_git_adapter/list_primitives_returns_alphabetic_summaries.md) |
| called_by | [primitive_saves_each_create_a_commit](/crates/oxide-library/tests/local_git_adapter/primitive_saves_each_create_a_commit.md) |
| called_by | [save_then_get_symbol_round_trip](/crates/oxide-library/tests/local_git_adapter/save_then_get_symbol_round_trip.md) |
