---
okf_version: "0.2"
type: Function
title: init_adapter
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/init_adapter
language: rust
---

# init_adapter

## Signature

```rust
fn init_adapter(
    dir: &tempfile::TempDir,
    name: &str,
    review_required: bool,
) -> (PathBuf, LocalGitAdapter)
```

## Source
Lines 55–72 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [empty_snx_manifest](/crates/oxide-library/tests/local_git_adapter/empty_snx_manifest.md) |
| called_by | [get_symbol_missing_uuid_is_not_found](/crates/oxide-library/tests/local_git_adapter/get_symbol_missing_uuid_is_not_found.md) |
| called_by | [history_returns_per_primitive_commits](/crates/oxide-library/tests/local_git_adapter/history_returns_per_primitive_commits.md) |
| called_by | [list_footprints_and_sims_are_name_sorted](/crates/oxide-library/tests/local_git_adapter/list_footprints_and_sims_are_name_sorted.md) |
| called_by | [list_primitives_rejects_zero_primitive_envelope](/crates/oxide-library/tests/local_git_adapter/list_primitives_rejects_zero_primitive_envelope.md) |
| called_by | [list_primitives_returns_alphabetic_summaries](/crates/oxide-library/tests/local_git_adapter/list_primitives_returns_alphabetic_summaries.md) |
| called_by | [list_primitives_skips_non_uuid_and_foreign_files](/crates/oxide-library/tests/local_git_adapter/list_primitives_skips_non_uuid_and_foreign_files.md) |
| called_by | [primitive_saves_each_create_a_commit](/crates/oxide-library/tests/local_git_adapter/primitive_saves_each_create_a_commit.md) |
| called_by | [save_then_get_footprint_round_trip](/crates/oxide-library/tests/local_git_adapter/save_then_get_footprint_round_trip.md) |
| called_by | [save_then_get_sim_round_trip](/crates/oxide-library/tests/local_git_adapter/save_then_get_sim_round_trip.md) |
| called_by | [save_then_get_symbol_round_trip](/crates/oxide-library/tests/local_git_adapter/save_then_get_symbol_round_trip.md) |
