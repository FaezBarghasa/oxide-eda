---
okf_version: "0.2"
type: Function
title: polygon_centroid
description: Centroid of a Polygon graphic — the shared anchor definition used
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/polygon_centroid
language: rust
---

# polygon_centroid

Centroid of a Polygon graphic — the shared anchor definition used

## Signature

```rust
pub fn polygon_centroid(vertices: &[[f64; 2]]) -> [f64; 2]
```

## Visibility

- `pub`

## Docstring

Centroid of a Polygon graphic — the shared anchor definition used
by canvas selection-anchor lookup, rotate-pivot geometry-center,
and whole-shape translate so all three agree on the same point.

Area-weighted (the standard shoelace centroid formula), NOT a
plain vertex mean: a joined polygon can carry far more vertices on
one side than another (e.g. a tessellated arc side contributes ~16
points, a straight side contributes 2), and a vertex mean skews
the "centre" toward whichever side happens to be more densely
subdivided — dragging or rotating the shape then pivots around a
point nowhere near its visual middle. Falls back to the vertex
mean when the polygon's signed area is ~zero (a bowtie or other
degenerate/self-intersecting ring, where the area-weighted formula
divides by ~zero) and for the empty list (should not occur —
placement always commits >= 3 vertices).

## Source
Lines 370–392 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| calls | [polygon_vertex_mean](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_vertex_mean.md) |
| called_by | [selection_anchor](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/selection_anchor.md) |
| called_by | [graphic_geometry_center](/crates/oxide-app/src/library/editor/symbol/state/rotation/graphic_geometry_center.md) |
| called_by | [translate_graphic_to](/crates/oxide-app/src/library/editor/symbol/state/rotation/translate_graphic_to.md) |
| called_by | [polygon_centroid_averages_vertices](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_centroid_averages_vertices.md) |
| called_by | [polygon_centroid_falls_back_to_vertex_mean_for_a_bowtie](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_centroid_falls_back_to_vertex_mean_for_a_bowtie.md) |
| called_by | [polygon_centroid_is_area_weighted_not_skewed_by_a_densely_subdivided_side](/crates/oxide-app/src/library/editor/symbol/state/tests/polygon_centroid_is_area_weighted_not_skewed_by_a_densely_subdivided_side.md) |
