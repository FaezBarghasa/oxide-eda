---
okf_version: "0.2"
type: Module
title: profile_tests
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests
language: rust
---

# profile_tests

## Relationships

| Type | Target |
|------|--------|
| related | [loads_built_in_profiles](/crates/oxide-app/src/keymap/profile_tests/loads_built_in_profiles.md) |
| related | [lookup_supports_pending_multi_stroke_sequences](/crates/oxide-app/src/keymap/profile_tests/lookup_supports_pending_multi_stroke_sequences.md) |
| related | [copies_built_in_profile_as_custom](/crates/oxide-app/src/keymap/profile_tests/copies_built_in_profile_as_custom.md) |
| related | [later_unbind_suppresses_earlier_command_binding](/crates/oxide-app/src/keymap/profile_tests/later_unbind_suppresses_earlier_command_binding.md) |
| related | [exports_and_imports_custom_profile_toml](/crates/oxide-app/src/keymap/profile_tests/exports_and_imports_custom_profile_toml.md) |
| related | [persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile](/crates/oxide-app/src/keymap/profile_tests/persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile.md) |
| related | [save_profile_set_at_leaves_previous_profiles_intact_when_write_fails](/crates/oxide-app/src/keymap/profile_tests/save_profile_set_at_leaves_previous_profiles_intact_when_write_fails.md) |
| related | [import_rejects_built_in_profile_documents](/crates/oxide-app/src/keymap/profile_tests/import_rejects_built_in_profile_documents.md) |
| related | [persistence_rejects_custom_profile_shadowing_built_in_id](/crates/oxide-app/src/keymap/profile_tests/persistence_rejects_custom_profile_shadowing_built_in_id.md) |
| related | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| related | [load_reports_error_when_the_file_cannot_be_parsed](/crates/oxide-app/src/keymap/profile_tests/load_reports_error_when_the_file_cannot_be_parsed.md) |
| related | [load_reports_error_when_the_active_profile_id_does_not_resolve](/crates/oxide-app/src/keymap/profile_tests/load_reports_error_when_the_active_profile_id_does_not_resolve.md) |
| related | [load_succeeds_when_the_file_is_absent](/crates/oxide-app/src/keymap/profile_tests/load_succeeds_when_the_file_is_absent.md) |
| related | [back_up_preserves_the_original_profiles](/crates/oxide-app/src/keymap/profile_tests/back_up_preserves_the_original_profiles.md) |
| related | [back_up_refuses_to_overwrite_an_existing_backup](/crates/oxide-app/src/keymap/profile_tests/back_up_refuses_to_overwrite_an_existing_backup.md) |
| related | [back_up_is_a_no_op_when_no_file_exists](/crates/oxide-app/src/keymap/profile_tests/back_up_is_a_no_op_when_no_file_exists.md) |
| related | [back_up_appends_bak_to_the_whole_file_name](/crates/oxide-app/src/keymap/profile_tests/back_up_appends_bak_to_the_whole_file_name.md) |
| related | [the_users_profiles_survive_an_apply_after_a_failed_load](/crates/oxide-app/src/keymap/profile_tests/the_users_profiles_survive_an_apply_after_a_failed_load.md) |
| related | [a_backup_whose_active_profile_is_gone_still_restores_its_profiles](/crates/oxide-app/src/keymap/profile_tests/a_backup_whose_active_profile_is_gone_still_restores_its_profiles.md) |
| related | [an_unparseable_backup_fails_the_restore_and_changes_nothing](/crates/oxide-app/src/keymap/profile_tests/an_unparseable_backup_fails_the_restore_and_changes_nothing.md) |
| related | [restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it](/crates/oxide-app/src/keymap/profile_tests/restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it.md) |
| related | [discarding_removes_the_backup_and_is_a_no_op_when_there_is_none](/crates/oxide-app/src/keymap/profile_tests/discarding_removes_the_backup_and_is_a_no_op_when_there_is_none.md) |
