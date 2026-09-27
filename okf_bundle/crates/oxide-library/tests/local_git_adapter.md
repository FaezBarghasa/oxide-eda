---
okf_version: "0.2"
type: Module
title: local_git_adapter
description: Integration tests for the local + git library adapter.
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter
language: rust
---

# local_git_adapter

Integration tests for the local + git library adapter.

## Docstring

Integration tests for the local + git library adapter.

Per `v0.9-refactor-2-plan.md` §7, this exercise covers both flows the
adapter ships:

* Primitive (`Symbol` / `Footprint` / `SimModel`) round-trip + commit.
* Row CRUD over `tables/<category>.tsv` — insert/update/delete by id,
lookup by `internal_pn`, and `iter_rows` across every table.

## Relationships

| Type | Target |
|------|--------|
| related | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
| related | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| related | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
| related | [init_open_round_trip_empty_library](/crates/oxide-library/tests/local_git_adapter/init_open_round_trip_empty_library.md) |
| related | [init_refuses_existing_library](/crates/oxide-library/tests/local_git_adapter/init_refuses_existing_library.md) |
| related | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
| related | [fixture_footprint](/crates/oxide-library/tests/local_git_adapter/fixture_footprint.md) |
| related | [fixture_sim](/crates/oxide-library/tests/local_git_adapter/fixture_sim.md) |
| related | [library_id_returns_manifest_id](/crates/oxide-library/tests/local_git_adapter/library_id_returns_manifest_id.md) |
| related | [save_then_get_symbol_round_trip](/crates/oxide-library/tests/local_git_adapter/save_then_get_symbol_round_trip.md) |
| related | [get_symbol_missing_uuid_is_not_found](/crates/oxide-library/tests/local_git_adapter/get_symbol_missing_uuid_is_not_found.md) |
| related | [save_then_get_footprint_round_trip](/crates/oxide-library/tests/local_git_adapter/save_then_get_footprint_round_trip.md) |
| related | [save_then_get_sim_round_trip](/crates/oxide-library/tests/local_git_adapter/save_then_get_sim_round_trip.md) |
| related | [primitive_saves_each_create_a_commit](/crates/oxide-library/tests/local_git_adapter/primitive_saves_each_create_a_commit.md) |
| related | [history_returns_per_primitive_commits](/crates/oxide-library/tests/local_git_adapter/history_returns_per_primitive_commits.md) |
| related | [list_primitives_returns_alphabetic_summaries](/crates/oxide-library/tests/local_git_adapter/list_primitives_returns_alphabetic_summaries.md) |
| related | [list_footprints_and_sims_are_name_sorted](/crates/oxide-library/tests/local_git_adapter/list_footprints_and_sims_are_name_sorted.md) |
| related | [list_primitives_skips_non_uuid_and_foreign_files](/crates/oxide-library/tests/local_git_adapter/list_primitives_skips_non_uuid_and_foreign_files.md) |
| related | [list_primitives_rejects_zero_primitive_envelope](/crates/oxide-library/tests/local_git_adapter/list_primitives_rejects_zero_primitive_envelope.md) |
| related | [library_set_resolves_across_two_local_libs](/crates/oxide-library/tests/local_git_adapter/library_set_resolves_across_two_local_libs.md) |
| related | [corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid](/crates/oxide-library/tests/local_git_adapter/corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid.md) |
| related | [fixture_row](/crates/oxide-library/tests/local_git_adapter/fixture_row.md) |
| related | [local_git_round_trip_row](/crates/oxide-library/tests/local_git_adapter/local_git_round_trip_row.md) |
| related | [local_git_iter_rows_across_tables](/crates/oxide-library/tests/local_git_adapter/local_git_iter_rows_across_tables.md) |
| related | [local_git_read_row_by_pn](/crates/oxide-library/tests/local_git_adapter/local_git_read_row_by_pn.md) |
| related | [local_git_commits_with_message](/crates/oxide-library/tests/local_git_adapter/local_git_commits_with_message.md) |
| related | [snx_manifest_with_mode](/crates/oxide-library/tests/local_git_adapter/snx_manifest_with_mode.md) |
| related | [cascade_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_personal_mode_auto_bumps_bound_row.md) |
| related | [cascade_team_mode_leaves_released_row_stale](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_leaves_released_row_stale.md) |
| related | [cascade_team_mode_unreleased_row_auto_bumps](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_unreleased_row_auto_bumps.md) |
| related | [cascade_footprint_personal_mode_auto_bumps_bound_row](/crates/oxide-library/tests/local_git_adapter/cascade_footprint_personal_mode_auto_bumps_bound_row.md) |
