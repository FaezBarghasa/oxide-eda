---
okf_version: "0.2"
type: Module
title: updates
description: Update logic for the standalone Footprint editor.
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod
language: rust
---

# updates

Update logic for the standalone Footprint editor.

## Docstring

Update logic for the standalone Footprint editor.

`apply_footprint_primitive_edit` is the router: the pre-match `msg`
rewrite, the exhaustive dispatch match, and the shared pad/undo helpers.
Each concern's arms live in a sibling module and are reached through one
`|`-grouped delegating arm per concern (ADR-0001 D1/D2). The former
monolithic `sketch` module is itself now split by sketch concern
into the `sketch/` folder (ui / placement / entities / pad_bridge /
constraints / tools):

sketch::{ui, placement, entities, pad_bridge, constraints, tools}
· active_bar · geometry · selection · context_menu · view

## Relationships

| Type | Target |
|------|--------|
| related | [align_pads](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_pads.md) |
| related | [apply_align](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_align.md) |
| related | [SpacingAxis](/crates/oxide-app/src/library/editor/footprint/updates/mod/SpacingAxis.md) |
| related | [scale_axis](/crates/oxide-app/src/library/editor/footprint/updates/mod/scale_axis.md) |
| related | [apply_footprint_clipboard_op](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_clipboard_op.md) |
| related | [apply_footprint_primitive_edit](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_primitive_edit.md) |
| related | [footprint_nudge_selection](/crates/oxide-app/src/library/editor/footprint/updates/mod/footprint_nudge_selection.md) |
| related | [mutates_footprint_state](/crates/oxide-app/src/library/editor/footprint/updates/mod/mutates_footprint_state.md) |
| related | [approx_eq](/crates/oxide-app/src/library/editor/footprint/updates/mod/approx_eq.md) |
| related | [align_left_moves_all_x_to_min](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_left_moves_all_x_to_min.md) |
| related | [align_right_moves_all_x_to_max](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_right_moves_all_x_to_max.md) |
| related | [align_top_bottom_move_y_only](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_top_bottom_move_y_only.md) |
| related | [center_h_v_align_to_mean](/crates/oxide-app/src/library/editor/footprint/updates/mod/center_h_v_align_to_mean.md) |
| related | [distribute_h_equalises_gaps_and_keeps_extremes](/crates/oxide-app/src/library/editor/footprint/updates/mod/distribute_h_equalises_gaps_and_keeps_extremes.md) |
| related | [distribute_h_preserves_input_order_when_unsorted](/crates/oxide-app/src/library/editor/footprint/updates/mod/distribute_h_preserves_input_order_when_unsorted.md) |
| related | [distribute_v_equalises_gaps](/crates/oxide-app/src/library/editor/footprint/updates/mod/distribute_v_equalises_gaps.md) |
| related | [increase_h_spacing_grows_span_by_step_times_gaps](/crates/oxide-app/src/library/editor/footprint/updates/mod/increase_h_spacing_grows_span_by_step_times_gaps.md) |
| related | [decrease_h_spacing_shrinks_span](/crates/oxide-app/src/library/editor/footprint/updates/mod/decrease_h_spacing_shrinks_span.md) |
| related | [decrease_spacing_clamps_at_zero_span](/crates/oxide-app/src/library/editor/footprint/updates/mod/decrease_spacing_clamps_at_zero_span.md) |
| related | [increase_v_spacing_grows_vertical_span](/crates/oxide-app/src/library/editor/footprint/updates/mod/increase_v_spacing_grows_vertical_span.md) |
