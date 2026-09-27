---
okf_version: "0.2"
type: Module
title: library_pad_actions
description: "Pad selection, clipboard, rotate/flip, courtyard recompute, and context-menu dispatch."
resource: crates/oxide-app/tests/regression/library_pad_actions.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_actions
language: rust
---

# library_pad_actions

Pad selection, clipboard, rotate/flip, courtyard recompute, and context-menu dispatch.

## Docstring

Pad selection, clipboard, rotate/flip, courtyard recompute, and context-menu dispatch.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture_footprint_with_pads](/crates/oxide-app/tests/regression/library_pad_actions/fixture_footprint_with_pads.md) |
| related | [v026e_copy_with_no_selection_is_noop](/crates/oxide-app/tests/regression/library_pad_actions/v026e_copy_with_no_selection_is_noop.md) |
| related | [v026e_copy_populates_clipboard_with_selected_pad](/crates/oxide-app/tests/regression/library_pad_actions/v026e_copy_populates_clipboard_with_selected_pad.md) |
| related | [v026e_cut_removes_pad_populates_clipboard_and_pushes_history](/crates/oxide-app/tests/regression/library_pad_actions/v026e_cut_removes_pad_populates_clipboard_and_pushes_history.md) |
| related | [v026e_paste_at_cursor_with_bumped_designator](/crates/oxide-app/tests/regression/library_pad_actions/v026e_paste_at_cursor_with_bumped_designator.md) |
| related | [v026e_paste_resets_sketch_entity_links](/crates/oxide-app/tests/regression/library_pad_actions/v026e_paste_resets_sketch_entity_links.md) |
| related | [v026e_paste_with_empty_clipboard_is_noop](/crates/oxide-app/tests/regression/library_pad_actions/v026e_paste_with_empty_clipboard_is_noop.md) |
| related | [v026g_rotate_selection_increments_rotation_by_90_degrees](/crates/oxide-app/tests/regression/library_pad_actions/v026g_rotate_selection_increments_rotation_by_90_degrees.md) |
| related | [v026g_rotate_selection_wraps_at_360](/crates/oxide-app/tests/regression/library_pad_actions/v026g_rotate_selection_wraps_at_360.md) |
| related | [v026g_flip_selection_swaps_top_to_bottom_layers](/crates/oxide-app/tests/regression/library_pad_actions/v026g_flip_selection_swaps_top_to_bottom_layers.md) |
| related | [v026g_rotate_with_no_selection_is_noop](/crates/oxide-app/tests/regression/library_pad_actions/v026g_rotate_with_no_selection_is_noop.md) |
| related | [v026c_fit_to_window_action_arms_fit_pending_and_closes_menu](/crates/oxide-app/tests/regression/library_pad_actions/v026c_fit_to_window_action_arms_fit_pending_and_closes_menu.md) |
| related | [v026c_fit_consumed_clears_fit_pending](/crates/oxide-app/tests/regression/library_pad_actions/v026c_fit_consumed_clears_fit_pending.md) |
| related | [v026i_auto_fit_courtyard_default_is_false_after_from_footprint](/crates/oxide-app/tests/regression/library_pad_actions/v026i_auto_fit_courtyard_default_is_false_after_from_footprint.md) |
| related | [v026i_recompute_courtyard_with_auto_fit_off_does_not_overwrite_courtyard](/crates/oxide-app/tests/regression/library_pad_actions/v026i_recompute_courtyard_with_auto_fit_off_does_not_overwrite_courtyard.md) |
| related | [v026i_recompute_courtyard_with_auto_fit_on_still_computes_pad_bbox](/crates/oxide-app/tests/regression/library_pad_actions/v026i_recompute_courtyard_with_auto_fit_on_still_computes_pad_bbox.md) |
| related | [v026b_show_context_menu_pad_target_selects_pad_and_clears_silk](/crates/oxide-app/tests/regression/library_pad_actions/v026b_show_context_menu_pad_target_selects_pad_and_clears_silk.md) |
| related | [v026d_show_context_menu_silk_target_selects_silk_and_clears_pad](/crates/oxide-app/tests/regression/library_pad_actions/v026d_show_context_menu_silk_target_selects_silk_and_clears_pad.md) |
| related | [v026b_show_context_menu_empty_target_preserves_selection_and_opens_menu](/crates/oxide-app/tests/regression/library_pad_actions/v026b_show_context_menu_empty_target_preserves_selection_and_opens_menu.md) |
