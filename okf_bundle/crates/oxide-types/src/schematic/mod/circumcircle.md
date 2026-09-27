---
okf_version: "0.2"
type: Function
title: circumcircle
description: Circle through three non-collinear points — converts the Oxide
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/circumcircle
language: rust
---

# circumcircle

Circle through three non-collinear points — converts the Oxide

## Signature

```rust
pub fn circumcircle(a: Point, b: Point, c: Point) -> Option<(f64, f64, f64)>
```

## Visibility

- `pub`

## Docstring

Circle through three non-collinear points — converts the Oxide
(start, mid, end) arc storage into (center, radius) for rendering,
hit-testing, and the properties-panel arc editor. Returns `None` when
`a`, `b`, `c` are (numerically) collinear, i.e. no finite circumcircle
exists.

Canonical home for this math: it used to be copied into 4 call sites
with a drifted collinearity epsilon (1e-9 in three places, 1e-12 in a
fourth) — a determinant landing in (1e-12, 1e-9) was a valid arc to one
path and "degenerate" (silently falling back to a wrong center) to the
others. `1e-12` is the deliberate choice here: it's the tighter, safer
"still solvable" threshold — `d` only needs to be far enough from exact
zero that dividing by it doesn't blow up, and schematic coordinates are
small enough (mm-scale) that `1e-9` was rejecting perfectly good arcs.

## Source
Lines 143–158 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
| called_by | [apply_drawing_edit](/crates/oxide-app/src/app/handlers/editing_commands/apply_drawing_edit.md) |
| called_by | [draw_arc_preview](/crates/oxide-app/src/canvas/draw/previews/draw_arc_preview.md) |
| called_by | [draw_sketch_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_sketch_tool_preview.md) |
| called_by | [edge_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/edge_arc.md) |
| called_by | [draw](/crates/oxide-app/src/panels/element_properties/drawing_preview/draw.md) |
| called_by | [drawing_aabb](/crates/oxide-app/src/schematic_runtime/mod/drawing_aabb.md) |
| called_by | [build_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/snapshot/build_renderer_snapshot.md) |
| called_by | [describe_single_selection](/crates/oxide-engine/src/selection/describe_single_selection.md) |
| called_by | [circle_from_three_points](/crates/oxide-output/src/svg/geometry/circle_from_three_points.md) |
| called_by | [solves_a_simple_right_triangle](/crates/oxide-types/src/schematic/mod/solves_a_simple_right_triangle.md) |
| called_by | [solves_near_collinear_triangle_in_the_drifted_epsilon_band](/crates/oxide-types/src/schematic/mod/solves_near_collinear_triangle_in_the_drifted_epsilon_band.md) |
