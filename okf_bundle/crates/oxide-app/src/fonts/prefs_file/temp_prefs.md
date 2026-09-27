---
okf_version: "0.2"
type: Function
title: temp_prefs
description: "A tempdir plus the `prefs.json` path inside it. Deliberately not"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/temp_prefs
language: rust
---

# temp_prefs

A tempdir plus the `prefs.json` path inside it. Deliberately not

## Signature

```rust
fn temp_prefs() -> (tempfile::TempDir, PathBuf)
```

## Docstring

A tempdir plus the `prefs.json` path inside it. Deliberately not
`prefs_path()`: that resolver caches in a process-wide
`OnceLock`, so tests sharing it would race under `cargo test`'s
default parallelism.

## Source
Lines 398–405 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| called_by | [absent_prefs_file_is_created](/crates/oxide-app/src/fonts/prefs_file/absent_prefs_file_is_created.md) |
| called_by | [checking_a_file_never_creates_or_modifies_it](/crates/oxide-app/src/fonts/prefs_file/checking_a_file_never_creates_or_modifies_it.md) |
| called_by | [checking_a_malformed_file_reports_a_parse_error](/crates/oxide-app/src/fonts/prefs_file/checking_a_malformed_file_reports_a_parse_error.md) |
| called_by | [checking_a_non_object_root_reports_it](/crates/oxide-app/src/fonts/prefs_file/checking_a_non_object_root_reports_it.md) |
| called_by | [checking_a_valid_object_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_a_valid_object_reports_it_as_healthy.md) |
| called_by | [checking_an_absent_file_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_an_absent_file_reports_it_as_healthy.md) |
| called_by | [checking_an_empty_file_reports_it_as_healthy](/crates/oxide-app/src/fonts/prefs_file/checking_an_empty_file_reports_it_as_healthy.md) |
| called_by | [empty_prefs_file_is_treated_as_absent](/crates/oxide-app/src/fonts/prefs_file/empty_prefs_file_is_treated_as_absent.md) |
| called_by | [existing_keys_survive_an_unrelated_write](/crates/oxide-app/src/fonts/prefs_file/existing_keys_survive_an_unrelated_write.md) |
| called_by | [malformed_prefs_file_is_left_byte_identical](/crates/oxide-app/src/fonts/prefs_file/malformed_prefs_file_is_left_byte_identical.md) |
| called_by | [moving_aside_appends_bak_to_the_whole_file_name](/crates/oxide-app/src/fonts/prefs_file/moving_aside_appends_bak_to_the_whole_file_name.md) |
| called_by | [moving_aside_is_a_no_op_when_there_is_no_file](/crates/oxide-app/src/fonts/prefs_file/moving_aside_is_a_no_op_when_there_is_no_file.md) |
| called_by | [moving_aside_is_not_undone_by_the_legacy_prefs_migration](/crates/oxide-app/src/fonts/prefs_file/moving_aside_is_not_undone_by_the_legacy_prefs_migration.md) |
| called_by | [moving_aside_leaves_the_prefs_path_loadable_again](/crates/oxide-app/src/fonts/prefs_file/moving_aside_leaves_the_prefs_path_loadable_again.md) |
| called_by | [moving_aside_never_overwrites_an_existing_backup](/crates/oxide-app/src/fonts/prefs_file/moving_aside_never_overwrites_an_existing_backup.md) |
| called_by | [moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence](/crates/oxide-app/src/fonts/prefs_file/moving_aside_reports_a_failure_it_cannot_tell_apart_from_absence.md) |
| called_by | [non_object_root_is_left_alone_and_does_not_panic](/crates/oxide-app/src/fonts/prefs_file/non_object_root_is_left_alone_and_does_not_panic.md) |
| called_by | [unreadable_prefs_file_is_left_alone](/crates/oxide-app/src/fonts/prefs_file/unreadable_prefs_file_is_left_alone.md) |
| called_by | [writes_resume_after_the_broken_file_is_moved_aside](/crates/oxide-app/src/fonts/prefs_file/writes_resume_after_the_broken_file_is_moved_aside.md) |
