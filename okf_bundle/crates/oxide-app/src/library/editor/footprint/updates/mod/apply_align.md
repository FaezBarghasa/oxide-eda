---
okf_version: "0.2"
type: Function
title: apply_align
description: "Pure geometry for [`align_pads`]: take pad CENTRES (in selection"
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod/apply_align
language: rust
---

# apply_align

Pure geometry for [`align_pads`]: take pad CENTRES (in selection

## Signature

```rust
fn apply_align(
    centres: &[(f64, f64)],
    op: crate::library::editor::footprint::state::AlignOp,
    step: f64,
) -> Vec<(f64, f64)>
```

## Docstring

Pure geometry for [`align_pads`]: take pad CENTRES (in selection
order), apply `op`, and return the new centres in the SAME order.
`step` is the spacing increment (mm) used only by the
Increase/Decrease ops. Factored out as a free function so the
geometry is unit-testable without a `FootprintEditorState`.

Conventions (Altium pad-centre parity):
- Left/Right/Top/Bottom move every centre to the extreme centre on
that axis (min/max). The cross axis is untouched, which is why the
plain and "maintain spacing" variants share an op.
- CenterH/CenterV move centres to the selection mean on that axis.
- DistributeH/V keep the two extreme pads fixed and re-space the
middles at equal centre-to-centre gaps, preserving left→right /
top→bottom order.
- Increase/Decrease grow/shrink every gap by `step` (span changes by
`step*(n-1)`), pivoting about the mean so the centroid is fixed.

## Source
Lines 65–132 in `crates/oxide-app/src/library/editor/footprint/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/footprint/updates/mod.md) |
| calls | [scale_axis](/crates/oxide-app/src/library/editor/footprint/updates/mod/scale_axis.md) |
| called_by | [align_left_moves_all_x_to_min](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_left_moves_all_x_to_min.md) |
| called_by | [align_pads](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_pads.md) |
| called_by | [align_right_moves_all_x_to_max](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_right_moves_all_x_to_max.md) |
| called_by | [align_top_bottom_move_y_only](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_top_bottom_move_y_only.md) |
| called_by | [center_h_v_align_to_mean](/crates/oxide-app/src/library/editor/footprint/updates/mod/center_h_v_align_to_mean.md) |
| called_by | [decrease_h_spacing_shrinks_span](/crates/oxide-app/src/library/editor/footprint/updates/mod/decrease_h_spacing_shrinks_span.md) |
| called_by | [decrease_spacing_clamps_at_zero_span](/crates/oxide-app/src/library/editor/footprint/updates/mod/decrease_spacing_clamps_at_zero_span.md) |
| called_by | [distribute_h_equalises_gaps_and_keeps_extremes](/crates/oxide-app/src/library/editor/footprint/updates/mod/distribute_h_equalises_gaps_and_keeps_extremes.md) |
| called_by | [distribute_h_preserves_input_order_when_unsorted](/crates/oxide-app/src/library/editor/footprint/updates/mod/distribute_h_preserves_input_order_when_unsorted.md) |
| called_by | [distribute_v_equalises_gaps](/crates/oxide-app/src/library/editor/footprint/updates/mod/distribute_v_equalises_gaps.md) |
| called_by | [increase_h_spacing_grows_span_by_step_times_gaps](/crates/oxide-app/src/library/editor/footprint/updates/mod/increase_h_spacing_grows_span_by_step_times_gaps.md) |
| called_by | [increase_v_spacing_grows_vertical_span](/crates/oxide-app/src/library/editor/footprint/updates/mod/increase_v_spacing_grows_vertical_span.md) |
