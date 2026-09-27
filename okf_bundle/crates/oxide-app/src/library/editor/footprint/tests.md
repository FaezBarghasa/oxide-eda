---
okf_version: "0.2"
type: Module
title: tests
description: Footprint editor tests.
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests
language: rust
---

# tests

Footprint editor tests.

## Docstring

Footprint editor tests.

Coverage focuses on the layer visibility helpers + the pad
hit-test surface; the smaller `from_footprint` / `add_pad_at` /
`sync_pads_to_primitive` flow is covered in `state.rs` itself.

## Relationships

| Type | Target |
|------|--------|
| related | [empty_state_has_no_pads_or_courtyard](/crates/oxide-app/src/library/editor/footprint/tests/empty_state_has_no_pads_or_courtyard.md) |
| related | [add_two_pads_then_hit_test](/crates/oxide-app/src/library/editor/footprint/tests/add_two_pads_then_hit_test.md) |
| related | [delete_pad_clears_selection](/crates/oxide-app/src/library/editor/footprint/tests/delete_pad_clears_selection.md) |
| related | [auto_fit_courtyard_tracks_pads](/crates/oxide-app/src/library/editor/footprint/tests/auto_fit_courtyard_tracks_pads.md) |
| related | [layer_visibility_default_only_front_on](/crates/oxide-app/src/library/editor/footprint/tests/layer_visibility_default_only_front_on.md) |
| related | [nudge_pads_translates_selection_by_delta](/crates/oxide-app/src/library/editor/footprint/tests/nudge_pads_translates_selection_by_delta.md) |
| related | [mint_body3d_extrudes_courtyard](/crates/oxide-app/src/library/editor/footprint/tests/mint_body3d_extrudes_courtyard.md) |
| related | [nudge_pads_skips_out_of_range_indices](/crates/oxide-app/src/library/editor/footprint/tests/nudge_pads_skips_out_of_range_indices.md) |
| related | [nudge_pads_empty_selection_is_noop](/crates/oxide-app/src/library/editor/footprint/tests/nudge_pads_empty_selection_is_noop.md) |
| related | [move_by_modal_nudges_by_typed_delta](/crates/oxide-app/src/library/editor/footprint/tests/move_by_modal_nudges_by_typed_delta.md) |
| related | [place_text_frame_sets_frame_box](/crates/oxide-app/src/library/editor/footprint/tests/place_text_frame_sets_frame_box.md) |
| related | [apply_filter_preset_sets_state_filter](/crates/oxide-app/src/library/editor/footprint/tests/apply_filter_preset_sets_state_filter.md) |
| related | [place_move_button_left_click_arms_select_tool](/crates/oxide-app/src/library/editor/footprint/tests/place_move_button_left_click_arms_select_tool.md) |
| related | [default_editor](/crates/oxide-app/src/library/editor/footprint/tests/default_editor.md) |
| related | [escape_closes_context_menu_before_clearing_selection](/crates/oxide-app/src/library/editor/footprint/tests/escape_closes_context_menu_before_clearing_selection.md) |
| related | [place_move_button](/crates/oxide-app/src/library/editor/footprint/tests/place_move_button.md) |
| related | [editor_with_pads](/crates/oxide-app/src/library/editor/footprint/tests/editor_with_pads.md) |
| related | [issue_146_rotate_with_no_selection_stays_clean](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_rotate_with_no_selection_stays_clean.md) |
| related | [issue_146_rotate_with_selection_dirties_and_snapshots_once](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_rotate_with_selection_dirties_and_snapshots_once.md) |
| related | [issue_146_align_to_grid_with_no_selection_stays_clean](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_align_to_grid_with_no_selection_stays_clean.md) |
| related | [issue_146_context_click_on_new_pad_clears_stale_extras](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_context_click_on_new_pad_clears_stale_extras.md) |
| related | [issue_146_context_select_all_fills_extras_like_active_bar](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_context_select_all_fills_extras_like_active_bar.md) |
| related | [editor_with_positions](/crates/oxide-app/src/library/editor/footprint/tests/editor_with_positions.md) |
| related | [align_confirm_horizontal_matches_direct_align_pads](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_horizontal_matches_direct_align_pads.md) |
| related | [align_confirm_both_axes_is_one_undo_step](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_both_axes_is_one_undo_step.md) |
| related | [align_confirm_both_axes_matches_two_sequential_align_pads](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_both_axes_matches_two_sequential_align_pads.md) |
| related | [align_confirm_neither_axis_is_clean_noop](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_neither_axis_is_clean_noop.md) |
| related | [align_confirm_below_size_gate_pushes_no_history](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_below_size_gate_pushes_no_history.md) |
| related | [align_cancel_closes_modal_and_keeps_selection](/crates/oxide-app/src/library/editor/footprint/tests/align_cancel_closes_modal_and_keeps_selection.md) |
