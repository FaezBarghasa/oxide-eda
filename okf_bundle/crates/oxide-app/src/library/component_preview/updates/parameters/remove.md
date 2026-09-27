---
okf_version: "0.2"
type: Function
title: remove
description: Drop a parameter from the row.
resource: crates/oxide-app/src/library/component_preview/updates/parameters.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/parameters/remove
language: rust
---

# remove

Drop a parameter from the row.

## Signature

```rust
pub(super) fn remove(state: &mut ComponentPreviewState, name: String)
```

## Visibility

- `pub(super)`

## Docstring

Drop a parameter from the row.

## Source
Lines 70–73 in `crates/oxide-app/src/library/component_preview/updates/parameters.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameters](/crates/oxide-app/src/library/component_preview/updates/parameters.md) |
| called_by | [dispatch_file_message](/crates/oxide-app/src/app/dispatch/document/dispatch_file_message.md) |
| called_by | [handle_browser_cell_cancel](/crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_cell_cancel.md) |
| called_by | [finish_open_library_browser](/crates/oxide-app/src/app/dispatch/library/browser/mod/finish_open_library_browser.md) |
| called_by | [handle_browser_cell_commit](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_cell_commit.md) |
| called_by | [handle_browser_edit_msg](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_edit_msg.md) |
| called_by | [handle_editor_event](/crates/oxide-app/src/app/dispatch/library/component_preview/handle_editor_event.md) |
| called_by | [handle_components_panel_promote_to_global](/crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_promote_to_global.md) |
| called_by | [save_primitive_tab_at](/crates/oxide-app/src/app/dispatch/library/editor/save_primitive_tab_at.md) |
| called_by | [handle_library_updates_apply](/crates/oxide-app/src/app/dispatch/library/updates/handle_library_updates_apply.md) |
| called_by | [scan_library_updates_for_open_schematic](/crates/oxide-app/src/app/dispatch/library/updates/scan_library_updates_for_open_schematic.md) |
| called_by | [dispatch_net_color_message](/crates/oxide-app/src/app/dispatch/mod/dispatch_net_color_message.md) |
| called_by | [dispatch_window_message](/crates/oxide-app/src/app/dispatch/mod/dispatch_window_message.md) |
| called_by | [dispatch_annotate_message](/crates/oxide-app/src/app/dispatch/overlay/dispatch_annotate_message.md) |
| called_by | [dispatch_erc_message](/crates/oxide-app/src/app/dispatch/overlay/dispatch_erc_message.md) |
| called_by | [handle_canvas_event_in_window](/crates/oxide-app/src/app/dispatch/ui/handle_canvas_event_in_window.md) |
| called_by | [push_history](/crates/oxide-app/src/app/documents/push_history.md) |
| called_by | [redo](/crates/oxide-app/src/app/documents/redo.md) |
| called_by | [undo](/crates/oxide-app/src/app/documents/undo.md) |
| called_by | [handle_active_bar_filter_toggle](/crates/oxide-app/src/app/handlers/active_bar/filter_controls/handle_active_bar_filter_toggle.md) |
| called_by | [handle_remove_custom_filter_preset](/crates/oxide-app/src/app/handlers/active_bar/filter_controls/handle_remove_custom_filter_preset.md) |
| called_by | [handle_toggle_custom_filter_preset_member](/crates/oxide-app/src/app/handlers/active_bar/filter_controls/handle_toggle_custom_filter_preset_member.md) |
| called_by | [handle_canvas_clicked](/crates/oxide-app/src/app/handlers/canvas/clicked/handle_canvas_clicked.md) |
| called_by | [handle_canvas_interaction_event](/crates/oxide-app/src/app/handlers/canvas/mod/handle_canvas_interaction_event.md) |
| called_by | [handle_dock_panel_control_message](/crates/oxide-app/src/app/handlers/dock/panel_controls/handle_dock_panel_control_message.md) |
| called_by | [handle_project_close_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/handle_project_close_confirm.md) |
| called_by | [try_save_dirty_path](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/try_save_dirty_path.md) |
| called_by | [handle_project_rename_submit](/crates/oxide-app/src/app/handlers/dock/project_navigation/rename/handle_project_rename_submit.md) |
| called_by | [handle_rename_submit](/crates/oxide-app/src/app/handlers/dock/project_navigation/rename/handle_rename_submit.md) |
| called_by | [handle_fp_editor_grid_manager_delete](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_grid_manager_delete.md) |
| called_by | [handle_fp_editor_guide_delete](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/grid/handle_fp_editor_guide_delete.md) |
| called_by | [handle_fp_library_delete_internal](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/library/handle_fp_library_delete_internal.md) |
| called_by | [handle_fp_editor_delete_selected_silk](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/silk/handle_fp_editor_delete_selected_silk.md) |
| called_by | [sch_library_delete_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_delete_symbol.md) |
| called_by | [handle_project_git_commit_done](/crates/oxide-app/src/app/handlers/document_files/git/handle_project_git_commit_done.md) |
| called_by | [handle_save_primitive_as](/crates/oxide-app/src/app/handlers/document_files/save/handle_save_primitive_as.md) |
| called_by | [persist_project_by_id](/crates/oxide-app/src/app/handlers/document_files/save/persist_project_by_id.md) |
| called_by | [close_tab_now](/crates/oxide-app/src/app/handlers/document_tabs/close_tab_now.md) |
| called_by | [handle_document_tab_message](/crates/oxide-app/src/app/handlers/document_tabs/handle_document_tab_message.md) |
| called_by | [handle_annotate](/crates/oxide-app/src/app/handlers/erc/annotate/handle_annotate.md) |
| called_by | [handle_reset_duplicate_designators](/crates/oxide-app/src/app/handlers/erc/annotate/handle_reset_duplicate_designators.md) |
| called_by | [close_detached_modal](/crates/oxide-app/src/app/handlers/erc/modals/close_detached_modal.md) |
| called_by | [handle_detach_floating_panel](/crates/oxide-app/src/app/handlers/erc/modals/handle_detach_floating_panel.md) |
| called_by | [handle_erc_severity_changed](/crates/oxide-app/src/app/handlers/erc/modals/handle_erc_severity_changed.md) |
| called_by | [handle_bom_preview_close](/crates/oxide-app/src/app/handlers/menu/export/bom/handle_bom_preview_close.md) |
| called_by | [handle_bom_preview_column_drag_drop](/crates/oxide-app/src/app/handlers/menu/export/bom/handle_bom_preview_column_drag_drop.md) |
| called_by | [handle_bom_preview_toggle_column](/crates/oxide-app/src/app/handlers/menu/export/bom/handle_bom_preview_toggle_column.md) |
| called_by | [handle_export_pdf_finished](/crates/oxide-app/src/app/handlers/menu/export/pdf_netlist/handle_export_pdf_finished.md) |
| called_by | [handle_print_preview_close](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_close.md) |
| called_by | [handle_print_preview_toggle_file](/crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_toggle_file.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
| called_by | [with_active_schematic_session_mut](/crates/oxide-app/src/app/load_gateway/with_active_schematic_session_mut.md) |
| called_by | [clear_active_engine](/crates/oxide-app/src/app/state/mod/clear_active_engine.md) |
| called_by | [preview_sheets](/crates/oxide-app/src/app/view/dialogs/annotate_preview/preview_sheets.md) |
| called_by | [update](/crates/oxide-app/src/dock/state/update.md) |
| called_by | [write_library_browser_search](/crates/oxide-app/src/fonts/misc/write_library_browser_search.md) |
| called_by | [forget_refusal](/crates/oxide-app/src/fonts/prefs_file/forget_refusal.md) |
| called_by | [edit_active_trigger](/crates/oxide-app/src/keymap/editor/edit_active_trigger.md) |
| called_by | [delete_custom_profile](/crates/oxide-app/src/keymap/profile/delete_custom_profile.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
| called_by | [set_pin_node](/crates/oxide-app/src/library/component_preview/updates/sim/set_pin_node.md) |
| called_by | [remove_alternate](/crates/oxide-app/src/library/component_preview/updates/supply/remove_alternate.md) |
| called_by | [remove_listing](/crates/oxide-app/src/library/component_preview/updates/supply/remove_listing.md) |
| called_by | [box_select_pads](/crates/oxide-app/src/library/editor/footprint/canvas/input/release/box_select_pads.md) |
| called_by | [try_pad_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_pad_grab.md) |
| called_by | [apply_edit_inner](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_edit_inner.md) |
| called_by | [delete_pad](/crates/oxide-app/src/library/editor/footprint/state/mod/delete_pad.md) |
| called_by | [place_move_button](/crates/oxide-app/src/library/editor/footprint/tests/place_move_button.md) |
| called_by | [delete_silk_f](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_silk_f.md) |
| called_by | [lasso_commit](/crates/oxide-app/src/library/editor/footprint/updates/selection/lasso_commit.md) |
| called_by | [select_all_on_layer](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_all_on_layer.md) |
| called_by | [select_off_grid_pads](/crates/oxide-app/src/library/editor/footprint/updates/selection/select_off_grid_pads.md) |
| called_by | [touching_line_commit](/crates/oxide-app/src/library/editor/footprint/updates/selection/touching_line_commit.md) |
| called_by | [set_role](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/set_role.md) |
| called_by | [input_tab](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_tab.md) |
| called_by | [delete_selected](/crates/oxide-app/src/library/editor/symbol/state/hit_test/delete_selected.md) |
| called_by | [splice_selection_into_polygon](/crates/oxide-app/src/library/editor/symbol/updates/join/splice_selection_into_polygon.md) |
| called_by | [push_undo_snapshot](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo_snapshot.md) |
| called_by | [take_mount_intent](/crates/oxide-app/src/library/mount/take_mount_intent.md) |
| called_by | [close_library](/crates/oxide-app/src/library/state/methods/close_library.md) |
| called_by | [exec_edits](/crates/oxide-engine/src/exec/edits/exec_edits.md) |
| called_by | [exec_structure](/crates/oxide-engine/src/exec/structure/exec_structure.md) |
| called_by | [record_history](/crates/oxide-engine/src/history/record_history.md) |
| called_by | [release](/crates/oxide-library-server/src/locks/release.md) |
| called_by | [unmount](/crates/oxide-library/src/adapters/library_set/unmount.md) |
| called_by | [unmount_by_path](/crates/oxide-library/src/adapters/library_set/unmount_by_path.md) |
| called_by | [delete_empty_table](/crates/oxide-library/src/adapters/local_git/adapter/delete_empty_table.md) |
| called_by | [rename_table](/crates/oxide-library/src/adapters/local_git/adapter/rename_table.md) |
| called_by | [drop_project](/crates/oxide-library/src/where_used/drop_project.md) |
| called_by | [ingest_sheet](/crates/oxide-library/src/where_used/ingest_sheet.md) |
| called_by | [build_bom_context](/crates/oxide-output/src/bom/mod/build_bom_context.md) |
| called_by | [update_simplex_and_direction](/crates/oxide-physics/src/collision/update_simplex_and_direction.md) |
| called_by | [unsubscribe](/crates/oxide-proto/src/mqtt/unsubscribe.md) |
| called_by | [cut_redundant_corners](/crates/oxide-router/src/optimization/glossing/cut_redundant_corners.md) |
| called_by | [remove_collinear_segments](/crates/oxide-router/src/optimization/glossing/remove_collinear_segments.md) |
| called_by | [ear_clip](/crates/oxide-sketch/src/geom/triangulate/ear_clip.md) |
| called_by | [commit_split](/crates/oxide-sketch/src/split/mod/commit_split.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
