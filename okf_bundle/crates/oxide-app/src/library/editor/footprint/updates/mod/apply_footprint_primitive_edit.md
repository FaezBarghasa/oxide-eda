---
okf_version: "0.2"
type: Function
title: apply_footprint_primitive_edit
description: Apply a primitive-editor event to a standalone Footprint editor
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_primitive_edit
language: rust
---

# apply_footprint_primitive_edit

Apply a primitive-editor event to a standalone Footprint editor

## Signature

```rust
pub(crate) fn apply_footprint_primitive_edit(
    editor: &mut crate::app::FootprintEditorState,
    msg: FootprintEditorMsg,
)
```

## Visibility

- `pub(crate)`

## Docstring

Apply a primitive-editor event to a standalone Footprint editor
state. Mirrors the footprint-tab arms of `apply_inline_edit` but
against the path-keyed standalone state.

## Source
Lines 291–444 in `crates/oxide-app/src/library/editor/footprint/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/footprint/updates/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [mutates_footprint_state](/crates/oxide-app/src/library/editor/footprint/updates/mod/mutates_footprint_state.md) |
| called_by | [handle_footprint_primitive_edit](/crates/oxide-app/src/app/dispatch/library/editor/handle_footprint_primitive_edit.md) |
| called_by | [break_track_click_near_endpoint_warns_no_split](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_click_near_endpoint_warns_no_split.md) |
| called_by | [break_track_miss_warns_and_leaves_line_intact](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_miss_warns_and_leaves_line_intact.md) |
| called_by | [break_track_reselects_line_a_not_line_b](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_reselects_line_a_not_line_b.md) |
| called_by | [break_track_split_at_mid_span_replaces_line_with_two_halves](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/break_track_split_at_mid_span_replaces_line_with_two_halves.md) |
| called_by | [click](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/click.md) |
| called_by | [angle_constraint_with_unit_suffix_is_reported_not_swallowed](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/angle_constraint_with_unit_suffix_is_reported_not_swallowed.md) |
| called_by | [constraint_that_does_not_match_the_selection_is_reported](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/constraint_that_does_not_match_the_selection_is_reported.md) |
| called_by | [distance_constraint_with_a_readable_dimension_still_applies](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/distance_constraint_with_a_readable_dimension_still_applies.md) |
| called_by | [distance_constraint_with_comma_decimal_is_reported_not_swallowed](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/distance_constraint_with_comma_decimal_is_reported_not_swallowed.md) |
| called_by | [fillet_with_an_empty_radius_still_uses_the_default](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/fillet_with_an_empty_radius_still_uses_the_default.md) |
| called_by | [fillet_with_an_unreadable_radius_creates_nothing](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/fillet_with_an_unreadable_radius_creates_nothing.md) |
| called_by | [offset_with_an_empty_distance_still_uses_the_default](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/offset_with_an_empty_distance_still_uses_the_default.md) |
| called_by | [offset_with_an_unreadable_distance_creates_nothing](/crates/oxide-app/src/library/editor/footprint/swallowed_input_tests/offset_with_an_unreadable_distance_creates_nothing.md) |
| called_by | [align_cancel_closes_modal_and_keeps_selection](/crates/oxide-app/src/library/editor/footprint/tests/align_cancel_closes_modal_and_keeps_selection.md) |
| called_by | [align_confirm_below_size_gate_pushes_no_history](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_below_size_gate_pushes_no_history.md) |
| called_by | [align_confirm_both_axes_is_one_undo_step](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_both_axes_is_one_undo_step.md) |
| called_by | [align_confirm_both_axes_matches_two_sequential_align_pads](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_both_axes_matches_two_sequential_align_pads.md) |
| called_by | [align_confirm_horizontal_matches_direct_align_pads](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_horizontal_matches_direct_align_pads.md) |
| called_by | [align_confirm_neither_axis_is_clean_noop](/crates/oxide-app/src/library/editor/footprint/tests/align_confirm_neither_axis_is_clean_noop.md) |
| called_by | [issue_146_align_to_grid_with_no_selection_stays_clean](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_align_to_grid_with_no_selection_stays_clean.md) |
| called_by | [issue_146_context_click_on_new_pad_clears_stale_extras](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_context_click_on_new_pad_clears_stale_extras.md) |
| called_by | [issue_146_context_select_all_fills_extras_like_active_bar](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_context_select_all_fills_extras_like_active_bar.md) |
| called_by | [issue_146_rotate_with_no_selection_stays_clean](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_rotate_with_no_selection_stays_clean.md) |
| called_by | [issue_146_rotate_with_selection_dirties_and_snapshots_once](/crates/oxide-app/src/library/editor/footprint/tests/issue_146_rotate_with_selection_dirties_and_snapshots_once.md) |
