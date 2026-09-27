---
okf_version: "0.2"
type: Function
title: point_xy
description: "Look up a Point's current `(x, y)` — either from the state vector"
resource: crates/oxide-sketch/src/solver/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/state/point_xy
language: rust
---

# point_xy

Look up a Point's current `(x, y)` — either from the state vector

## Signature

```rust
pub fn point_xy(
    id: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Option<(f64, f64)>
```

## Visibility

- `pub`

## Docstring

Look up a Point's current `(x, y)` — either from the state vector
(free variable) or directly from the [`Entity`] (Fixed-constrained).

## Source
Lines 70–85 in `crates/oxide-sketch/src/solver/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-sketch/src/solver/state.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
| called_by | [entity_center_xy](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_center_xy.md) |
| called_by | [entity_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_radius.md) |
| called_by | [line_length](/crates/oxide-sketch/src/solver/residuals/equal_tangent/line_length.md) |
| called_by | [tangent_line_arc](/crates/oxide-sketch/src/solver/residuals/equal_tangent/tangent_line_arc.md) |
| called_by | [line_dir](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/line_dir.md) |
| called_by | [distance_pt_circle](/crates/oxide-sketch/src/solver/residuals/point_on/distance_pt_circle.md) |
| called_by | [point_and_line](/crates/oxide-sketch/src/solver/residuals/point_on/point_and_line.md) |
| called_by | [point_on_arc](/crates/oxide-sketch/src/solver/residuals/point_on/point_on_arc.md) |
| called_by | [midpoint](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/midpoint.md) |
| called_by | [symmetric_about_line](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_line.md) |
| called_by | [symmetric_about_point](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_point.md) |
| called_by | [split_then_solve_leaves_rectangle_visually_unchanged](/crates/oxide-sketch/src/split/tests/solver/split_then_solve_leaves_rectangle_visually_unchanged.md) |
| called_by | [isosceles_triangle_apex_60](/crates/oxide-sketch/tests/canonical_sketches/isosceles_triangle_apex_60.md) |
| called_by | [parallelogram_base10_side5_60deg](/crates/oxide-sketch/tests/canonical_sketches/parallelogram_base10_side5_60deg.md) |
| called_by | [rectangle_10_by_5](/crates/oxide-sketch/tests/canonical_sketches/rectangle_10_by_5.md) |
| called_by | [regular_hexagon_circumradius_10](/crates/oxide-sketch/tests/canonical_sketches/regular_hexagon_circumradius_10.md) |
| called_by | [lm_already_converged_returns_quickly](/crates/oxide-sketch/tests/lm_basic/lm_already_converged_returns_quickly.md) |
| called_by | [lm_no_constraints_returns_immediately](/crates/oxide-sketch/tests/lm_basic/lm_no_constraints_returns_immediately.md) |
| called_by | [lm_solves_anchored_distance_in_either_direction](/crates/oxide-sketch/tests/lm_basic/lm_solves_anchored_distance_in_either_direction.md) |
| called_by | [lm_solves_anchored_horizontal_distance](/crates/oxide-sketch/tests/lm_basic/lm_solves_anchored_horizontal_distance.md) |
| called_by | [solver_default_solves_anchored_horizontal_distance](/crates/oxide-sketch/tests/solver_api/solver_default_solves_anchored_horizontal_distance.md) |
| called_by | [pack_excludes_fixed_points](/crates/oxide-sketch/tests/solver_basics/pack_excludes_fixed_points.md) |
| called_by | [pack_two_points_lays_out_xy_in_order](/crates/oxide-sketch/tests/solver_basics/pack_two_points_lays_out_xy_in_order.md) |
