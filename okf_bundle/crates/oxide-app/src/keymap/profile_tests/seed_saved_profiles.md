---
okf_version: "0.2"
type: Function
title: seed_saved_profiles
description: "Write a shortcuts file at `path` holding one custom profile, and"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles
language: rust
---

# seed_saved_profiles

Write a shortcuts file at `path` holding one custom profile, and

## Signature

```rust
fn seed_saved_profiles(path: &std::path::Path, profile_id: &str) -> Vec<u8>
```

## Docstring

Write a shortcuts file at `path` holding one custom profile, and
return the exact bytes that landed on disk. Shared by the backup
tests so each one starts from a real, schema-valid saved document
rather than a hand-written blob that rots against the format.

## Source
Lines 226–236 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [save_profile_set_at](/crates/oxide-app/src/keymap/profile/save_profile_set_at.md) |
| called_by | [a_backup_whose_active_profile_is_gone_still_restores_its_profiles](/crates/oxide-app/src/keymap/profile_tests/a_backup_whose_active_profile_is_gone_still_restores_its_profiles.md) |
| called_by | [an_unparseable_backup_fails_the_restore_and_changes_nothing](/crates/oxide-app/src/keymap/profile_tests/an_unparseable_backup_fails_the_restore_and_changes_nothing.md) |
| called_by | [back_up_appends_bak_to_the_whole_file_name](/crates/oxide-app/src/keymap/profile_tests/back_up_appends_bak_to_the_whole_file_name.md) |
| called_by | [back_up_preserves_the_original_profiles](/crates/oxide-app/src/keymap/profile_tests/back_up_preserves_the_original_profiles.md) |
| called_by | [back_up_refuses_to_overwrite_an_existing_backup](/crates/oxide-app/src/keymap/profile_tests/back_up_refuses_to_overwrite_an_existing_backup.md) |
| called_by | [discarding_removes_the_backup_and_is_a_no_op_when_there_is_none](/crates/oxide-app/src/keymap/profile_tests/discarding_removes_the_backup_and_is_a_no_op_when_there_is_none.md) |
| called_by | [load_reports_error_when_the_active_profile_id_does_not_resolve](/crates/oxide-app/src/keymap/profile_tests/load_reports_error_when_the_active_profile_id_does_not_resolve.md) |
| called_by | [restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it](/crates/oxide-app/src/keymap/profile_tests/restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it.md) |
| called_by | [the_users_profiles_survive_an_apply_after_a_failed_load](/crates/oxide-app/src/keymap/profile_tests/the_users_profiles_survive_an_apply_after_a_failed_load.md) |
