---
okf_version: "0.2"
type: Module
title: project
description: "Project/document lifecycle: modals, git pipeline, exit guard, open-gating."
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project
language: rust
---

# project

Project/document lifecycle: modals, git pipeline, exit guard, open-gating.

## Docstring

Project/document lifecycle: modals, git pipeline, exit guard, open-gating.

## Relationships

| Type | Target |
|------|--------|
| related | [oxide_new_constructs_with_default_state](/crates/oxide-app/tests/regression/project/oxide_new_constructs_with_default_state.md) |
| related | [fixture_project_with_companions](/crates/oxide-app/tests/regression/project/fixture_project_with_companions.md) |
| related | [arm_project_rename](/crates/oxide-app/tests/regression/project/arm_project_rename.md) |
| related | [arm_remove_dialog](/crates/oxide-app/tests/regression/project/arm_remove_dialog.md) |
| related | [f6_project_rename_does_not_touch_companion_snxsch_snxpcb](/crates/oxide-app/tests/regression/project/f6_project_rename_does_not_touch_companion_snxsch_snxpcb.md) |
| related | [f6_project_rename_refuses_to_overwrite_existing_target](/crates/oxide-app/tests/regression/project/f6_project_rename_refuses_to_overwrite_existing_target.md) |
| related | [f6_project_rename_rejects_path_separators_in_buffer](/crates/oxide-app/tests/regression/project/f6_project_rename_rejects_path_separators_in_buffer.md) |
| related | [f6_project_rename_with_unchanged_stem_is_a_silent_noop](/crates/oxide-app/tests/regression/project/f6_project_rename_with_unchanged_stem_is_a_silent_noop.md) |
| related | [remove_with_delete_choice_unlinks_the_file](/crates/oxide-app/tests/regression/project/remove_with_delete_choice_unlinks_the_file.md) |
| related | [remove_with_exclude_choice_keeps_the_file_on_disk](/crates/oxide-app/tests/regression/project/remove_with_exclude_choice_keeps_the_file_on_disk.md) |
| related | [f10_save_clears_dirty_paths_and_refreshes_panel_ctx](/crates/oxide-app/tests/regression/project/f10_save_clears_dirty_paths_and_refreshes_panel_ctx.md) |
| related | [f10_save_persists_snxprj_as_valid_json](/crates/oxide-app/tests/regression/project/f10_save_persists_snxprj_as_valid_json.md) |
| related | [add_existing_same_file_twice_is_silently_skipped](/crates/oxide-app/tests/regression/project/add_existing_same_file_twice_is_silently_skipped.md) |
| related | [add_existing_with_external_path_copies_into_project_dir](/crates/oxide-app/tests/regression/project/add_existing_with_external_path_copies_into_project_dir.md) |
| related | [project_rename_migrates_dirty_paths_to_new_path](/crates/oxide-app/tests/regression/project/project_rename_migrates_dirty_paths_to_new_path.md) |
| related | [add_new_schematic_writes_blank_snxsch_marks_project_dirty_no_tab_open](/crates/oxide-app/tests/regression/project/add_new_schematic_writes_blank_snxsch_marks_project_dirty_no_tab_open.md) |
| related | [add_new_schematic_cancelled_picker_is_a_clean_noop](/crates/oxide-app/tests/regression/project/add_new_schematic_cancelled_picker_is_a_clean_noop.md) |
| related | [project_options_modal_opens_with_metadata_then_closes](/crates/oxide-app/tests/regression/project/project_options_modal_opens_with_metadata_then_closes.md) |
| related | [rename_buffer_changed_updates_modal_buffer](/crates/oxide-app/tests/regression/project/rename_buffer_changed_updates_modal_buffer.md) |
| related | [close_rename_dialog_dismisses_modal_without_filesystem_changes](/crates/oxide-app/tests/regression/project/close_rename_dialog_dismisses_modal_without_filesystem_changes.md) |
| related | [close_remove_dialog_dismisses_modal_without_filesystem_changes](/crates/oxide-app/tests/regression/project/close_remove_dialog_dismisses_modal_without_filesystem_changes.md) |
| related | [f13_register_pending_library_does_not_touch_disk](/crates/oxide-app/tests/regression/project/f13_register_pending_library_does_not_touch_disk.md) |
| related | [f13_register_pending_rejects_existing_path](/crates/oxide-app/tests/regression/project/f13_register_pending_rejects_existing_path.md) |
| related | [f13_register_pending_rejects_non_snxlib_extension](/crates/oxide-app/tests/regression/project/f13_register_pending_rejects_non_snxlib_extension.md) |
| related | [loaded_project_data_round_trips_via_write_then_parse](/crates/oxide-app/tests/regression/project/loaded_project_data_round_trips_via_write_then_parse.md) |
| related | [project_git_commit_done_clears_inflight_entry](/crates/oxide-app/tests/regression/project/project_git_commit_done_clears_inflight_entry.md) |
| related | [commit_save_to_project_git_skips_when_enable_git_off](/crates/oxide-app/tests/regression/project/commit_save_to_project_git_skips_when_enable_git_off.md) |
| related | [commit_save_to_project_git_enqueues_when_enable_git_on](/crates/oxide-app/tests/regression/project/commit_save_to_project_git_enqueues_when_enable_git_on.md) |
| related | [app_exit_with_no_dirty_paths_does_not_open_confirm_modal](/crates/oxide-app/tests/regression/project/app_exit_with_no_dirty_paths_does_not_open_confirm_modal.md) |
| related | [app_exit_with_dirty_paths_opens_confirm_modal_instead_of_exiting](/crates/oxide-app/tests/regression/project/app_exit_with_dirty_paths_opens_confirm_modal_instead_of_exiting.md) |
| related | [app_exit_confirm_cancel_dismisses_modal_and_keeps_dirty_state](/crates/oxide-app/tests/regression/project/app_exit_confirm_cancel_dismisses_modal_and_keeps_dirty_state.md) |
| related | [app_exit_confirm_discard_all_clears_modal](/crates/oxide-app/tests/regression/project/app_exit_confirm_discard_all_clears_modal.md) |
| related | [app_exit_save_all_never_loses_an_unsaveable_file](/crates/oxide-app/tests/regression/project/app_exit_save_all_never_loses_an_unsaveable_file.md) |
| related | [new_project_over_existing_non_empty_snxprj_is_refused](/crates/oxide-app/tests/regression/project/new_project_over_existing_non_empty_snxprj_is_refused.md) |
| related | [write_valid_snxfpt](/crates/oxide-app/tests/regression/project/write_valid_snxfpt.md) |
| related | [write_valid_snxsym](/crates/oxide-app/tests/regression/project/write_valid_snxsym.md) |
| related | [opening_snxfpt_does_not_create_editable_tab_when_gated](/crates/oxide-app/tests/regression/project/opening_snxfpt_does_not_create_editable_tab_when_gated.md) |
| related | [opening_snxsym_still_creates_editable_tab](/crates/oxide-app/tests/regression/project/opening_snxsym_still_creates_editable_tab.md) |
| related | [a_corrupt_snxsym_raises_the_error_card_naming_the_file](/crates/oxide-app/tests/regression/project/a_corrupt_snxsym_raises_the_error_card_naming_the_file.md) |
| related | [a_valid_snxsym_leaves_the_error_card_clear](/crates/oxide-app/tests/regression/project/a_valid_snxsym_leaves_the_error_card_clear.md) |
| related | [dismissing_the_card_clears_it](/crates/oxide-app/tests/regression/project/dismissing_the_card_clears_it.md) |
| related | [fixture_schematic_with_symbol_and_child_sheet](/crates/oxide-app/tests/regression/project/fixture_schematic_with_symbol_and_child_sheet.md) |
| related | [cut_leaves_non_cuttable_child_sheet_in_place_and_selected](/crates/oxide-app/tests/regression/project/cut_leaves_non_cuttable_child_sheet_in_place_and_selected.md) |
| related | [save_all_writes_dirty_snxprj_and_clears_dirty_marker](/crates/oxide-app/tests/regression/project/save_all_writes_dirty_snxprj_and_clears_dirty_marker.md) |
