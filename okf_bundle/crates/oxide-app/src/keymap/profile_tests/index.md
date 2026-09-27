# profile_tests

## Functions

- [a_backup_whose_active_profile_is_gone_still_restores_its_profiles](a_backup_whose_active_profile_is_gone_still_restores_its_profiles.md) — The most likely reason the file failed to load is a dangling
- [an_unparseable_backup_fails_the_restore_and_changes_nothing](an_unparseable_backup_fails_the_restore_and_changes_nothing.md) — An unparseable backup is not recoverable, and the restore has to say
- [back_up_appends_bak_to_the_whole_file_name](back_up_appends_bak_to_the_whole_file_name.md) — Guards the `OsString` handling: the backup appends to the whole file
- [back_up_is_a_no_op_when_no_file_exists](back_up_is_a_no_op_when_no_file_exists.md) — No file, nothing to preserve — and no stray `.bak` left behind.
- [back_up_preserves_the_original_profiles](back_up_preserves_the_original_profiles.md) — The copy aside must be byte-identical, so the custom profiles this
- [back_up_refuses_to_overwrite_an_existing_backup](back_up_refuses_to_overwrite_an_existing_backup.md) — The double-Apply hazard: the second Apply would copy the already
- [copies_built_in_profile_as_custom](copies_built_in_profile_as_custom.md) — [test]
- [discarding_removes_the_backup_and_is_a_no_op_when_there_is_none](discarding_removes_the_backup_and_is_a_no_op_when_there_is_none.md) — Deleting is an explicit action and nothing else may do it — a
- [exports_and_imports_custom_profile_toml](exports_and_imports_custom_profile_toml.md) — [test]
- [import_rejects_built_in_profile_documents](import_rejects_built_in_profile_documents.md) — [test]
- [later_unbind_suppresses_earlier_command_binding](later_unbind_suppresses_earlier_command_binding.md) — [test]
- [load_reports_error_when_the_active_profile_id_does_not_resolve](load_reports_error_when_the_active_profile_id_does_not_resolve.md) — A well-formed file whose `active_profile` no longer resolves fails in
- [load_reports_error_when_the_file_cannot_be_parsed](load_reports_error_when_the_file_cannot_be_parsed.md) — A shortcuts file that exists but cannot be parsed must surface as
- [load_succeeds_when_the_file_is_absent](load_succeeds_when_the_file_is_absent.md) — The non-error path must stay non-error: a missing file is a fresh
- [loads_built_in_profiles](loads_built_in_profiles.md) — [test]
- [lookup_supports_pending_multi_stroke_sequences](lookup_supports_pending_multi_stroke_sequences.md) — [test]
- [persistence_rejects_custom_profile_shadowing_built_in_id](persistence_rejects_custom_profile_shadowing_built_in_id.md) — [test]
- [persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile](persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile.md) — [test]
- [restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it](restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it.md) — The `.bak` is never cleaned up, so it outlives the failure that made
- [save_profile_set_at_leaves_previous_profiles_intact_when_write_fails](save_profile_set_at_leaves_previous_profiles_intact_when_write_fails.md) — `save_profile_set_at` must go through `atomic_write`, not `fs::write`:
- [seed_saved_profiles](seed_saved_profiles.md) — Write a shortcuts file at `path` holding one custom profile, and
- [the_users_profiles_survive_an_apply_after_a_failed_load](the_users_profiles_survive_an_apply_after_a_failed_load.md) — The whole of #595 end to end, in the order the user hits it: a
