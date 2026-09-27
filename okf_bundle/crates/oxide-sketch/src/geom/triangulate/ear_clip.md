---
okf_version: "0.2"
type: Function
title: ear_clip
description: "Triangulate a simple polygon. Returns a list of triangles, each"
resource: crates/oxide-sketch/src/geom/triangulate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/triangulate/ear_clip
language: rust
---

# ear_clip

Triangulate a simple polygon. Returns a list of triangles, each

## Signature

```rust
pub fn ear_clip(polygon: &[Point2]) -> Vec<[usize; 3]>
```

## Visibility

- `pub`

## Docstring

Triangulate a simple polygon. Returns a list of triangles, each
as a triple of indices into the original `polygon` slice.

Convention:
- Input must be a closed simple polygon (no last-equals-first
duplicate).
- Winding is normalised internally — CW input gets a logical
reversal so the ear-detection always sees CCW.
- Output triangles are in the input's original index space and
in the input's original winding order.

Returns an empty Vec for fewer than three vertices, or when the
polygon is self-intersecting / has collinear vertices that make
every candidate degenerate.

## Source
Lines 27–105 in `crates/oxide-sketch/src/geom/triangulate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulate](/crates/oxide-sketch/src/geom/triangulate.md) |
| calls | [signed_area](/crates/oxide-sketch/src/geom/predicates/signed_area.md) |
| calls | [is_ear](/crates/oxide-sketch/src/geom/triangulate/is_ear.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [append_fill](/crates/oxide-gfx/src/pipeline/polygon/append_fill.md) |
| called_by | [concave_l_shape](/crates/oxide-sketch/src/geom/triangulate/concave_l_shape.md) |
| called_by | [convex_pentagon_emits_three_triangles](/crates/oxide-sketch/src/geom/triangulate/convex_pentagon_emits_three_triangles.md) |
| called_by | [convex_quad_cw_winding_still_works](/crates/oxide-sketch/src/geom/triangulate/convex_quad_cw_winding_still_works.md) |
| called_by | [convex_quad_emits_two_triangles](/crates/oxide-sketch/src/geom/triangulate/convex_quad_emits_two_triangles.md) |
| called_by | [degenerate_zero_area_returns_empty](/crates/oxide-sketch/src/geom/triangulate/degenerate_zero_area_returns_empty.md) |
| called_by | [ear_clip_partitions_l_shape_area_exactly](/crates/oxide-sketch/src/geom/triangulate/ear_clip_partitions_l_shape_area_exactly.md) |
| called_by | [ear_clip_with_holes](/crates/oxide-sketch/src/geom/triangulate/ear_clip_with_holes.md) |
| called_by | [triangle_emits_one_triangle](/crates/oxide-sketch/src/geom/triangulate/triangle_emits_one_triangle.md) |
