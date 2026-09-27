---
okf_version: "0.2"
type: Module
title: triangulate
description: Polygon triangulation via ear-clipping.
resource: crates/oxide-sketch/src/geom/triangulate.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/triangulate
language: rust
---

# triangulate

Polygon triangulation via ear-clipping.

## Docstring

Polygon triangulation via ear-clipping.

Time complexity: O(n²) for the basic form; with holes
pre-merged via the bridge-edge technique it stays O((n+h)²)
where h is the total hole-vertex count. Typical sketch closed
loops have ≤ ~64 vertices (rounded rect = 8 + 4·arc_segments),
where the basic form's constant factors beat the more
sophisticated O(n log n) variants.

## Relationships

| Type | Target |
|------|--------|
| related | [ear_clip](/crates/oxide-sketch/src/geom/triangulate/ear_clip.md) |
| related | [is_ear](/crates/oxide-sketch/src/geom/triangulate/is_ear.md) |
| related | [point_in_triangle](/crates/oxide-sketch/src/geom/triangulate/point_in_triangle.md) |
| related | [ear_clip_with_holes](/crates/oxide-sketch/src/geom/triangulate/ear_clip_with_holes.md) |
| related | [bridge_hole_into_outer](/crates/oxide-sketch/src/geom/triangulate/bridge_hole_into_outer.md) |
| related | [p](/crates/oxide-sketch/src/geom/triangulate/p.md) |
| related | [empty_polygon_no_triangles](/crates/oxide-sketch/src/geom/triangulate/empty_polygon_no_triangles.md) |
| related | [two_vertex_polygon_no_triangles](/crates/oxide-sketch/src/geom/triangulate/two_vertex_polygon_no_triangles.md) |
| related | [triangle_emits_one_triangle](/crates/oxide-sketch/src/geom/triangulate/triangle_emits_one_triangle.md) |
| related | [convex_quad_emits_two_triangles](/crates/oxide-sketch/src/geom/triangulate/convex_quad_emits_two_triangles.md) |
| related | [convex_quad_cw_winding_still_works](/crates/oxide-sketch/src/geom/triangulate/convex_quad_cw_winding_still_works.md) |
| related | [concave_l_shape](/crates/oxide-sketch/src/geom/triangulate/concave_l_shape.md) |
| related | [ear_clip_partitions_l_shape_area_exactly](/crates/oxide-sketch/src/geom/triangulate/ear_clip_partitions_l_shape_area_exactly.md) |
| related | [convex_pentagon_emits_three_triangles](/crates/oxide-sketch/src/geom/triangulate/convex_pentagon_emits_three_triangles.md) |
| related | [degenerate_zero_area_returns_empty](/crates/oxide-sketch/src/geom/triangulate/degenerate_zero_area_returns_empty.md) |
| related | [ear_clip_with_holes_no_holes_falls_through](/crates/oxide-sketch/src/geom/triangulate/ear_clip_with_holes_no_holes_falls_through.md) |
| related | [ear_clip_with_one_hole_emits_more_triangles](/crates/oxide-sketch/src/geom/triangulate/ear_clip_with_one_hole_emits_more_triangles.md) |
