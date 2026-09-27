---
okf_version: "0.2"
type: Function
title: approx_eq
description: Compare two centre lists with an absolute tolerance.
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod/approx_eq
language: rust
---

# approx_eq

Compare two centre lists with an absolute tolerance.

## Signature

```rust
fn approx_eq(a: &[(f64, f64)], b: &[(f64, f64)])
```

## Docstring

Compare two centre lists with an absolute tolerance.

## Source
Lines 614–622 in `crates/oxide-app/src/library/editor/footprint/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/footprint/updates/mod.md) |
| called_by | [align_left_moves_all_x_to_min](/crates/oxide-app/src/library/editor/footprint/updates/mod/align_left_moves_all_x_to_min.md) |
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
