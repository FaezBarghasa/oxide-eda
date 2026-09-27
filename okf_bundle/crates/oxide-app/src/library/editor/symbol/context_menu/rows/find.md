---
okf_version: "0.2"
type: Function
title: find
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/find
language: rust
---

# find

## Signature

```rust
fn find(rows: &'a [SymbolMenuRow], id: &str) -> &'a SymbolMenuRow
```

## Type Parameters

- `'a`

## Source
Lines 144–148 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
| called_by | [disabled_of](/crates/oxide-app/src/active_bar/dropdown/disabled_of.md) |
| called_by | [increment_designator](/crates/oxide-app/src/app/actions/increment_designator.md) |
| called_by | [open_overlays](/crates/oxide-app/src/app/bootstrap/subscription/open_overlays.md) |
| called_by | [action_for_id](/crates/oxide-app/src/app/command/active_bar/action_for_id.md) |
| called_by | [catalog_labels_match_the_active_bar_literals](/crates/oxide-app/src/app/command/active_bar/catalog_labels_match_the_active_bar_literals.md) |
| called_by | [id_for_action](/crates/oxide-app/src/app/command/active_bar/id_for_action.md) |
| called_by | [bridged_command_ids](/crates/oxide-app/src/app/command/bridge/bridged_command_ids.md) |
| called_by | [match_block](/crates/oxide-app/src/app/command/bridge/match_block.md) |
| called_by | [a_bound_command_row_shows_its_shortcut](/crates/oxide-app/src/app/command_palette/a_bound_command_row_shows_its_shortcut.md) |
| called_by | [focus_symbol_by_reference](/crates/oxide-app/src/app/dispatch/command_palette/focus_symbol_by_reference.md) |
| called_by | [handle_browser_begin_rename_class](/crates/oxide-app/src/app/dispatch/library/browser/classes/handle_browser_begin_rename_class.md) |
| called_by | [handle_browser_cell_commit](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_cell_commit.md) |
| called_by | [handle_browser_delete_row_request](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_delete_row_request.md) |
| called_by | [handle_browser_open_edit_modal](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_open_edit_modal.md) |
| called_by | [commit_external_change_for](/crates/oxide-app/src/app/dispatch/library/editor/commit_external_change_for.md) |
| called_by | [refresh_primitive_cache_for](/crates/oxide-app/src/app/dispatch/library/editor/refresh_primitive_cache_for.md) |
| called_by | [save_primitive_tab_at](/crates/oxide-app/src/app/dispatch/library/editor/save_primitive_tab_at.md) |
| called_by | [handle_new_component_set_table](/crates/oxide-app/src/app/dispatch/library/new_component/handle_new_component_set_table.md) |
| called_by | [apply_primitive_pick_to_browser_row](/crates/oxide-app/src/app/dispatch/library/primitive_picker/apply_primitive_pick_to_browser_row.md) |
| called_by | [handle_primitive_picker_browse_result](/crates/oxide-app/src/app/dispatch/library/primitive_picker/handle_primitive_picker_browse_result.md) |
| called_by | [handle_create_library_at_path](/crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_at_path.md) |
| called_by | [handle_create_library_for_project](/crates/oxide-app/src/app/dispatch/library/registration/handle_create_library_for_project.md) |
| called_by | [handle_picker_message](/crates/oxide-app/src/app/dispatch/library/registration/handle_picker_message.md) |
| called_by | [handle_library_updates_apply](/crates/oxide-app/src/app/dispatch/library/updates/handle_library_updates_apply.md) |
| called_by | [scan_library_updates_for_open_schematic](/crates/oxide-app/src/app/dispatch/library/updates/scan_library_updates_for_open_schematic.md) |
| called_by | [dispatch_window_message](/crates/oxide-app/src/app/dispatch/mod/dispatch_window_message.md) |
| called_by | [handle_active_bar_action](/crates/oxide-app/src/app/handlers/active_bar/action_groups/handle_active_bar_action.md) |
| called_by | [handle_canvas_double_clicked](/crates/oxide-app/src/app/handlers/canvas/double_clicked/handle_canvas_double_clicked.md) |
| called_by | [open_selected_child_sheet](/crates/oxide-app/src/app/handlers/canvas/mod/open_selected_child_sheet.md) |
| called_by | [primary_anchor_world](/crates/oxide-app/src/app/handlers/canvas/mod/primary_anchor_world.md) |
| called_by | [detached_panel_window](/crates/oxide-app/src/app/handlers/dock/panel_open/detached_panel_window.md) |
| called_by | [handle_project_close_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/handle_project_close_confirm.md) |
| called_by | [try_save_dirty_path](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/try_save_dirty_path.md) |
| called_by | [open_project_tree_document](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/open_project_tree_document.md) |
| called_by | [tree_path_to_file_path](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/tree_path_to_file_path.md) |
| called_by | [handle_enable_version_control_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/handle_enable_version_control_confirm.md) |
| called_by | [with_selected_sketch_pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_sketch_pad.md) |
| called_by | [fp_editor_edit_array_param](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_edit_array_param.md) |
| called_by | [fp_editor_set_array_numbering_scheme](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_array_numbering_scheme.md) |
| called_by | [fp_editor_set_bga_skip_letters](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_skip_letters.md) |
| called_by | [fp_editor_set_bga_start_col](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_col.md) |
| called_by | [fp_editor_set_bga_start_row](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_row.md) |
| called_by | [fp_editor_set_cutout_edge_radius](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_cutout_edge_radius.md) |
| called_by | [fp_editor_set_cutout_through](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_cutout_through.md) |
| called_by | [fp_editor_set_keepout_kind](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_keepout_kind.md) |
| called_by | [fp_editor_set_pour_fill_type](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_fill_type.md) |
| called_by | [fp_editor_set_pour_net](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_net.md) |
| called_by | [fp_editor_set_pour_priority](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_priority.md) |
| called_by | [fp_editor_toggle_array_instance](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_toggle_array_instance.md) |
| called_by | [mark_active_symbol_tab_dirty](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/mark_active_symbol_tab_dirty.md) |
| called_by | [sch_library_add_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_add_symbol.md) |
| called_by | [sch_library_delete_symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sch_library_delete_symbol.md) |
| called_by | [commit_save_to_project_git](/crates/oxide-app/src/app/handlers/document_files/git/commit_save_to_project_git.md) |
| called_by | [handle_history_restore_clicked](/crates/oxide-app/src/app/handlers/document_files/history/handle_history_restore_clicked.md) |
| called_by | [load_or_activate_project](/crates/oxide-app/src/app/handlers/document_files/open/load_or_activate_project.md) |
| called_by | [attach_library_for_path](/crates/oxide-app/src/app/handlers/document_files/save/attach_library_for_path.md) |
| called_by | [persist_project_by_id](/crates/oxide-app/src/app/handlers/document_files/save/persist_project_by_id.md) |
| called_by | [save_active_project_if_dirty](/crates/oxide-app/src/app/handlers/document_files/save/save_active_project_if_dirty.md) |
| called_by | [save_project_at_path](/crates/oxide-app/src/app/handlers/document_files/save/save_project_at_path.md) |
| called_by | [handle_update_drawing_field](/crates/oxide-app/src/app/handlers/editing_commands/handle_update_drawing_field.md) |
| called_by | [load_project_dsl_eval_fns](/crates/oxide-app/src/app/handlers/erc/erc_run/load_project_dsl_eval_fns.md) |
| called_by | [handle_menu_file_command](/crates/oxide-app/src/app/handlers/menu/file_commands/handle_menu_file_command.md) |
| called_by | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
| called_by | [expand_to_net](/crates/oxide-app/src/app/handlers/selection_workflow/expand_to_net.md) |
| called_by | [passes_filter](/crates/oxide-app/src/app/handlers/selection_workflow/passes_filter.md) |
| called_by | [build_footprint_editor_panel_ctx](/crates/oxide-app/src/app/runtime/footprint_ctx/build_footprint_editor_panel_ctx.md) |
| called_by | [build_sketch_entity_summary](/crates/oxide-app/src/app/runtime/footprint_summaries/build_sketch_entity_summary.md) |
| called_by | [update_selection_info](/crates/oxide-app/src/app/runtime/mod/update_selection_info.md) |
| called_by | [compute_library_row_detail](/crates/oxide-app/src/app/runtime/panel_ctx/compute_library_row_detail.md) |
| called_by | [project_by_id](/crates/oxide-app/src/app/state/mod/project_by_id.md) |
| called_by | [project_for_path](/crates/oxide-app/src/app/state/mod/project_for_path.md) |
| called_by | [project_listing_sheet](/crates/oxide-app/src/app/state/scope/project_listing_sheet.md) |
| called_by | [sheet_display_title](/crates/oxide-app/src/app/view/dialogs/annotate_preview/sheet_display_title.md) |
| called_by | [proposed_for](/crates/oxide-app/src/app/view/dialogs/annotate_preview/tests/proposed_for.md) |
| called_by | [dock_drag_zone_overlay](/crates/oxide-app/src/app/view/overlays/mod/dock_drag_zone_overlay.md) |
| called_by | [view_hover_tooltip](/crates/oxide-app/src/app/view/overlays/mod/view_hover_tooltip.md) |
| called_by | [draw_move_guides](/crates/oxide-app/src/canvas/draw/drag/draw_move_guides.md) |
| called_by | [draw_autofocus_dim](/crates/oxide-app/src/canvas/draw/scene/draw_autofocus_dim.md) |
| called_by | [extract_bracket_items](/crates/oxide-app/src/diagnostics/extract_bracket_items.md) |
| called_by | [extract_named_field](/crates/oxide-app/src/diagnostics/extract_named_field.md) |
| called_by | [group_of](/crates/oxide-app/src/keymap/catalog/mod/group_of.md) |
| called_by | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
| called_by | [apply_active_trigger](/crates/oxide-app/src/keymap/editor/apply_active_trigger.md) |
| called_by | [editor_rows_mark_pointer_gestures_as_not_keyboard_editable](/crates/oxide-app/src/keymap/editor_tests/editor_rows_mark_pointer_gestures_as_not_keyboard_editable.md) |
| called_by | [switching_active_profile_marks_editor_as_differing](/crates/oxide-app/src/keymap/editor_tests/switching_active_profile_marks_editor_as_differing.md) |
| called_by | [ids_from_call](/crates/oxide-app/src/keymap/menu_command_tests/ids_from_call.md) |
| called_by | [shortcut_label](/crates/oxide-app/src/keymap/profile/shortcut_label.md) |
| called_by | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
| called_by | [view_preview_pane](/crates/oxide-app/src/library/browser/preview/view_preview_pane.md) |
| called_by | [add_override](/crates/oxide-app/src/library/component_preview/updates/pin_map/add_override.md) |
| called_by | [open_override_edit](/crates/oxide-app/src/library/component_preview/updates/pin_map/open_override_edit.md) |
| called_by | [view](/crates/oxide-app/src/library/edit_row_modal/view.md) |
| called_by | [draw_select_cursor_mark](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_select_cursor_mark.md) |
| called_by | [arc_refs_local](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/arc_refs_local.md) |
| called_by | [circle_center_local](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/circle_center_local.md) |
| called_by | [draw_constraint_icons](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/draw_constraint_icons.md) |
| called_by | [draw_sketch_overlay](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/draw_sketch_overlay.md) |
| called_by | [point_world](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/point_world.md) |
| called_by | [draw_filled_closed_loops](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/draw_filled_closed_loops.md) |
| called_by | [find_closed_loops](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/find_closed_loops.md) |
| called_by | [point_pos](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/point_pos.md) |
| called_by | [pos](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/pos.md) |
| called_by | [draw_sketch_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_sketch_tool_preview.md) |
| called_by | [placement_field_buf](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/placement_field_buf.md) |
| called_by | [sketch_hit_other](/crates/oxide-app/src/library/editor/footprint/canvas/hit_test/sketch_hit_other.md) |
| called_by | [drag_tick_line](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/drag_tick_line.md) |
| called_by | [box_select_sketch](/crates/oxide-app/src/library/editor/footprint/canvas/input/release/box_select_sketch.md) |
| called_by | [try_drag_track_end_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_drag_track_end_grab.md) |
| called_by | [try_sketch_line_grab](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_sketch_line_grab.md) |
| called_by | [mouse_interaction](/crates/oxide-app/src/library/editor/footprint/canvas/mod/mouse_interaction.md) |
| called_by | [hit_test_uses_solved_positions_not_stale_authored_coords](/crates/oxide-app/src/library/editor/footprint/canvas/tests/hit_test_uses_solved_positions_not_stale_authored_coords.md) |
| called_by | [ensure_board_top_plane](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/ensure_board_top_plane.md) |
| called_by | [mirror_pad_attrs_into_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_pad_attrs_into_sketch.md) |
| called_by | [set_point_xy](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/set_point_xy.md) |
| called_by | [point_xy_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/point_xy_of.md) |
| called_by | [profile_seed_line](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/profile_seed_line.md) |
| called_by | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
| called_by | [persisted_ledger](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/persisted_ledger.md) |
| called_by | [record_ledger](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/record_ledger.md) |
| called_by | [copy_entity_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/copy_entity_geometry.md) |
| called_by | [pair_sidecar_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/pair_sidecar_entities.md) |
| called_by | [mirror_solve_to_round_rect_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_round_rect_geometry.md) |
| called_by | [in_place_remint_records_the_ledger_against_the_real_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/in_place_remint_records_the_ledger_against_the_real_sketch.md) |
| called_by | [minted_pad_attr_carries_the_rotation](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/minted_pad_attr_carries_the_rotation.md) |
| called_by | [mirror_add_pad_links_to_new_sketch_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_add_pad_links_to_new_sketch_entity.md) |
| called_by | [mirror_move_pad_updates_sketch_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_pad_updates_sketch_point.md) |
| called_by | [mirror_move_profile_pad_translates_profile_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_translates_profile_geometry.md) |
| called_by | [mirror_move_roundrect_translates_anchors_and_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_roundrect_translates_anchors_and_arc_centres.md) |
| called_by | [owned_point_positions](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_point_positions.md) |
| called_by | [owned_set_excludes_ids_with_no_live_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_set_excludes_ids_with_no_live_entity.md) |
| called_by | [point_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/point_of.md) |
| called_by | [roundrect_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/roundrect_arc_centres.md) |
| called_by | [shape_change_preserves_corner_positions](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/shape_change_preserves_corner_positions.md) |
| called_by | [sync_overwrites_every_bare_literal_form](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_overwrites_every_bare_literal_form.md) |
| called_by | [sync_preserves_a_bare_parameter_binding_with_no_eq_prefix](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_preserves_a_bare_parameter_binding_with_no_eq_prefix.md) |
| called_by | [sync_preserves_an_authored_rotation_expression](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sync_preserves_an_authored_rotation_expression.md) |
| called_by | [set_entity_role](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/set_entity_role.md) |
| called_by | [add_constraint_solves_geometry](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/add_constraint_solves_geometry.md) |
| called_by | [break_track_reselects_line_a_not_line_b](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_reselects_line_a_not_line_b.md) |
| called_by | [point_xy](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/point_xy.md) |
| called_by | [set_role_pad_on_line_is_silent_noop](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_pad_on_line_is_silent_noop.md) |
| called_by | [set_role_silk_top_attaches_silk_attr_with_top_layer](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_silk_top_attaches_silk_attr_with_top_layer.md) |
| called_by | [constraint_enable_matrix](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/constraint_enable_matrix.md) |
| called_by | [view_constraint_submenu](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_constraint_submenu.md) |
| called_by | [view_role](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_role.md) |
| called_by | [point_pos](/crates/oxide-app/src/library/editor/footprint/snap/point_pos.md) |
| called_by | [apply_footprint_primitive_edit](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_primitive_edit.md) |
| called_by | [add_constraint_for_selection](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/add_constraint_for_selection.md) |
| called_by | [move_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_line.md) |
| called_by | [move_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_point.md) |
| called_by | [make_pad_from_profile](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/make_pad_from_profile.md) |
| called_by | [set_role](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/set_role.md) |
| called_by | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| called_by | [edge_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/edge_arc.md) |
| called_by | [line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/line.md) |
| called_by | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| called_by | [rounded_rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rounded_rectangle.md) |
| called_by | [tangent_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/tangent_arc.md) |
| called_by | [break_track](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/break_track.md) |
| called_by | [fillet_second_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_second_click.md) |
| called_by | [line_xy](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/line_xy.md) |
| called_by | [pick_line_and_param](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_and_param.md) |
| called_by | [pick_line_at](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at.md) |
| called_by | [trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/trim.md) |
| called_by | [resolve_effective_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_effective_click.md) |
| called_by | [try_consume_repick_polar_center](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/try_consume_repick_polar_center.md) |
| called_by | [mirror](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/mirror.md) |
| called_by | [offset](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset.md) |
| called_by | [offset_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_arc.md) |
| called_by | [offset_circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_circle.md) |
| called_by | [offset_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_line.md) |
| called_by | [resolve_template](/crates/oxide-app/src/library/editor/params/resolve_template.md) |
| called_by | [pin_map_subsection](/crates/oxide-app/src/library/editor/preview/pin_map_subsection.md) |
| called_by | [top_level_ids_are_stable](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/top_level_ids_are_stable.md) |
| called_by | [delete_unit_prunes_and_renumbers_graphics](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_prunes_and_renumbers_graphics.md) |
| called_by | [delete_unit_removes_and_renumbers](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_removes_and_renumbers.md) |
| called_by | [polygon_is_collinear](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_is_collinear.md) |
| called_by | [view](/crates/oxide-app/src/library/new_component/view.md) |
| called_by | [containing_library](/crates/oxide-app/src/library/state/methods/containing_library.md) |
| called_by | [containing_library_mut](/crates/oxide-app/src/library/state/methods/containing_library_mut.md) |
| called_by | [library_at](/crates/oxide-app/src/library/state/methods/library_at.md) |
| called_by | [library_at_mut](/crates/oxide-app/src/library/state/methods/library_at_mut.md) |
| called_by | [toggle](/crates/oxide-app/src/library/updates_dialog/toggle.md) |
| called_by | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
| called_by | [view_drawing_properties](/crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties.md) |
| called_by | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
| called_by | [view_sch_library](/crates/oxide-app/src/panels/library/view_sch_library.md) |
| called_by | [view_navigator](/crates/oxide-app/src/panels/projects/view_navigator.md) |
| called_by | [form_grid_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_grid_row.md) |
| called_by | [form_grid_size_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_grid_size_row.md) |
| called_by | [view_pin_selection](/crates/oxide-app/src/panels/symbol_editor_properties/pin/view_pin_selection.md) |
| called_by | [view_pin_symbol_picker](/crates/oxide-app/src/panels/symbol_editor_properties/pin/view_pin_symbol_picker.md) |
| called_by | [content_keyboard_shortcuts](/crates/oxide-app/src/preferences/keymap/content_keyboard_shortcuts.md) |
| called_by | [pipeline_impl_block](/crates/oxide-app/src/scene_shader/pipeline_impl_block.md) |
| called_by | [hit_bus](/crates/oxide-app/src/schematic_runtime/hit_test/hit_bus.md) |
| called_by | [hit_wire](/crates/oxide-app/src/schematic_runtime/hit_test/hit_wire.md) |
| called_by | [item_aabb](/crates/oxide-app/src/schematic_runtime/mod/item_aabb.md) |
| called_by | [symbol_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_position.md) |
| called_by | [symbol_reference_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_reference_position.md) |
| called_by | [symbol_value_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_value_position.md) |
| called_by | [centre_point](/crates/oxide-app/tests/footprint_pad_remint/centre_point.md) |
| called_by | [flip_keeps_the_baked_shape_equal_to_the_editor_shape](/crates/oxide-app/tests/footprint_pad_remint/flip_keeps_the_baked_shape_equal_to_the_editor_shape.md) |
| called_by | [horizontal_edge_at_y](/crates/oxide-app/tests/footprint_pad_remint/horizontal_edge_at_y.md) |
| called_by | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
| called_by | [assert_corners_match_pad](/crates/oxide-app/tests/footprint_pad_rotation/assert_corners_match_pad.md) |
| called_by | [sketch_edge_drag_resizes_a_rotated_pad](/crates/oxide-app/tests/footprint_pad_rotation/sketch_edge_drag_resizes_a_rotated_pad.md) |
| called_by | [issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper.md) |
| called_by | [issue142_move_repairs_drifted_bbox_corners](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_move_repairs_drifted_bbox_corners.md) |
| called_by | [point_xy](/crates/oxide-app/tests/footprint_pad_sketch_mirror/point_xy.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
| called_by | [pcb_open_finished_ok_opens_the_tab_like_the_old_sync_path_did](/crates/oxide-app/tests/regression/file_open_async/pcb_open_finished_ok_opens_the_tab_like_the_old_sync_path_did.md) |
| called_by | [schematic_open_finished_ok_opens_the_tab_like_the_old_sync_path_did](/crates/oxide-app/tests/regression/file_open_async/schematic_open_finished_ok_opens_the_tab_like_the_old_sync_path_did.md) |
| called_by | [chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts](/crates/oxide-app/tests/regression/library_cross_track/chamfered_pad_with_2_enabled_corners_has_2_chamfer_cuts.md) |
| called_by | [tangent_arc_after_line_creates_tangent_constraint](/crates/oxide-app/tests/regression/library_cross_track/tangent_arc_after_line_creates_tangent_constraint.md) |
| called_by | [type_5_during_line_draw_commits_at_5mm](/crates/oxide-app/tests/regression/library_cross_track/type_5_during_line_draw_commits_at_5mm.md) |
| called_by | [editing_chamfer_len_propagates_through_solve](/crates/oxide-app/tests/regression/library_pad_geometry/editing_chamfer_len_propagates_through_solve.md) |
| called_by | [mirror_add_oval_pad_mints_2_arcs_2_lines_with_w_and_h_params](/crates/oxide-app/tests/regression/library_pad_geometry/mirror_add_oval_pad_mints_2_arcs_2_lines_with_w_and_h_params.md) |
| called_by | [mirror_add_round_rect_pad_mints_4_arcs_linked_to_corner_r](/crates/oxide-app/tests/regression/library_pad_geometry/mirror_add_round_rect_pad_mints_4_arcs_linked_to_corner_r.md) |
| called_by | [properties_panel_shows_corner_radius_for_round_rect_pad](/crates/oxide-app/tests/regression/library_pad_geometry/properties_panel_shows_corner_radius_for_round_rect_pad.md) |
| called_by | [unlink_corner_radius_mints_per_corner_param](/crates/oxide-app/tests/regression/library_pad_geometry/unlink_corner_radius_mints_per_corner_param.md) |
| called_by | [placement_input_circle_radius_pins_typed_radius](/crates/oxide-app/tests/regression/library_placement/placement_input_circle_radius_pins_typed_radius.md) |
| called_by | [placement_input_line_length_and_angle_commit_at_polar_offset](/crates/oxide-app/tests/regression/library_placement/placement_input_line_length_and_angle_commit_at_polar_offset.md) |
| called_by | [placement_input_line_length_pins_second_click_at_exact_distance](/crates/oxide-app/tests/regression/library_placement/placement_input_line_length_pins_second_click_at_exact_distance.md) |
| called_by | [placement_input_rounded_rect_commits_typed_size_and_radius](/crates/oxide-app/tests/regression/library_placement/placement_input_rounded_rect_commits_typed_size_and_radius.md) |
| called_by | [tangent_arc_tool_second_click_mints_arc_and_tangent_constraint](/crates/oxide-app/tests/regression/library_placement/tangent_arc_tool_second_click_mints_arc_and_tangent_constraint.md) |
| called_by | [v025_offset_placement_input_pins_typed_distance_over_cursor](/crates/oxide-app/tests/regression/library_placement/v025_offset_placement_input_pins_typed_distance_over_cursor.md) |
| called_by | [v027_sketch_line_drag_resizes_rect_pad_bbox](/crates/oxide-app/tests/regression/library_placement/v027_sketch_line_drag_resizes_rect_pad_bbox.md) |
| called_by | [one_gateway_edit_then_one_bypassing_edit](/crates/oxide-app/tests/regression/undo_marker_divergence/one_gateway_edit_then_one_bypassing_edit.md) |
| called_by | [primitive_file_sizes](/crates/oxide-app/tests/support/mod/primitive_file_sizes.md) |
