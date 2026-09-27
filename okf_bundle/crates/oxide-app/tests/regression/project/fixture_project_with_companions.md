---
okf_version: "0.2"
type: Function
title: fixture_project_with_companions
description: "Project skeleton: writes `<stem>.snxprj` + companion"
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project/fixture_project_with_companions
language: rust
---

# fixture_project_with_companions

Project skeleton: writes `<stem>.snxprj` + companion

## Signature

```rust
fn fixture_project_with_companions(stem: &str) -> (Oxide, TempDir, PathBuf)
```

## Docstring

Project skeleton: writes `<stem>.snxprj` + companion
`<stem>.snxsch` + `<stem>.snxpcb` into a fresh tempdir and
returns a populated `Oxide` with the project loaded.

## Source
Lines 44–80 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| called_by | [add_existing_same_file_twice_is_silently_skipped](/crates/oxide-app/tests/regression/project/add_existing_same_file_twice_is_silently_skipped.md) |
| called_by | [add_existing_with_external_path_copies_into_project_dir](/crates/oxide-app/tests/regression/project/add_existing_with_external_path_copies_into_project_dir.md) |
| called_by | [add_new_schematic_cancelled_picker_is_a_clean_noop](/crates/oxide-app/tests/regression/project/add_new_schematic_cancelled_picker_is_a_clean_noop.md) |
| called_by | [add_new_schematic_writes_blank_snxsch_marks_project_dirty_no_tab_open](/crates/oxide-app/tests/regression/project/add_new_schematic_writes_blank_snxsch_marks_project_dirty_no_tab_open.md) |
| called_by | [close_remove_dialog_dismisses_modal_without_filesystem_changes](/crates/oxide-app/tests/regression/project/close_remove_dialog_dismisses_modal_without_filesystem_changes.md) |
| called_by | [close_rename_dialog_dismisses_modal_without_filesystem_changes](/crates/oxide-app/tests/regression/project/close_rename_dialog_dismisses_modal_without_filesystem_changes.md) |
| called_by | [commit_save_to_project_git_enqueues_when_enable_git_on](/crates/oxide-app/tests/regression/project/commit_save_to_project_git_enqueues_when_enable_git_on.md) |
| called_by | [commit_save_to_project_git_skips_when_enable_git_off](/crates/oxide-app/tests/regression/project/commit_save_to_project_git_skips_when_enable_git_off.md) |
| called_by | [f10_save_clears_dirty_paths_and_refreshes_panel_ctx](/crates/oxide-app/tests/regression/project/f10_save_clears_dirty_paths_and_refreshes_panel_ctx.md) |
| called_by | [f10_save_persists_snxprj_as_valid_json](/crates/oxide-app/tests/regression/project/f10_save_persists_snxprj_as_valid_json.md) |
| called_by | [f6_project_rename_does_not_touch_companion_snxsch_snxpcb](/crates/oxide-app/tests/regression/project/f6_project_rename_does_not_touch_companion_snxsch_snxpcb.md) |
| called_by | [f6_project_rename_refuses_to_overwrite_existing_target](/crates/oxide-app/tests/regression/project/f6_project_rename_refuses_to_overwrite_existing_target.md) |
| called_by | [f6_project_rename_rejects_path_separators_in_buffer](/crates/oxide-app/tests/regression/project/f6_project_rename_rejects_path_separators_in_buffer.md) |
| called_by | [f6_project_rename_with_unchanged_stem_is_a_silent_noop](/crates/oxide-app/tests/regression/project/f6_project_rename_with_unchanged_stem_is_a_silent_noop.md) |
| called_by | [new_project_over_existing_non_empty_snxprj_is_refused](/crates/oxide-app/tests/regression/project/new_project_over_existing_non_empty_snxprj_is_refused.md) |
| called_by | [project_git_commit_done_clears_inflight_entry](/crates/oxide-app/tests/regression/project/project_git_commit_done_clears_inflight_entry.md) |
| called_by | [project_options_modal_opens_with_metadata_then_closes](/crates/oxide-app/tests/regression/project/project_options_modal_opens_with_metadata_then_closes.md) |
| called_by | [project_rename_migrates_dirty_paths_to_new_path](/crates/oxide-app/tests/regression/project/project_rename_migrates_dirty_paths_to_new_path.md) |
| called_by | [remove_with_delete_choice_unlinks_the_file](/crates/oxide-app/tests/regression/project/remove_with_delete_choice_unlinks_the_file.md) |
| called_by | [remove_with_exclude_choice_keeps_the_file_on_disk](/crates/oxide-app/tests/regression/project/remove_with_exclude_choice_keeps_the_file_on_disk.md) |
| called_by | [rename_buffer_changed_updates_modal_buffer](/crates/oxide-app/tests/regression/project/rename_buffer_changed_updates_modal_buffer.md) |
| called_by | [save_all_writes_dirty_snxprj_and_clears_dirty_marker](/crates/oxide-app/tests/regression/project/save_all_writes_dirty_snxprj_and_clears_dirty_marker.md) |
