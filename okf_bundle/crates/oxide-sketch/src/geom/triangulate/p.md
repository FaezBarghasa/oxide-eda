---
okf_version: "0.2"
type: Function
title: p
resource: crates/oxide-sketch/src/geom/triangulate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/triangulate/p
language: rust
---

# p

## Signature

```rust
fn p(x: f64, y: f64) -> Point2
```

## Source
Lines 288–290 in `crates/oxide-sketch/src/geom/triangulate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulate](/crates/oxide-sketch/src/geom/triangulate.md) |
| called_by | [concave_l_shape](/crates/oxide-sketch/src/geom/triangulate/concave_l_shape.md) |
| called_by | [convex_quad_cw_winding_still_works](/crates/oxide-sketch/src/geom/triangulate/convex_quad_cw_winding_still_works.md) |
| called_by | [convex_quad_emits_two_triangles](/crates/oxide-sketch/src/geom/triangulate/convex_quad_emits_two_triangles.md) |
| called_by | [degenerate_zero_area_returns_empty](/crates/oxide-sketch/src/geom/triangulate/degenerate_zero_area_returns_empty.md) |
| called_by | [ear_clip_partitions_l_shape_area_exactly](/crates/oxide-sketch/src/geom/triangulate/ear_clip_partitions_l_shape_area_exactly.md) |
| called_by | [triangle_emits_one_triangle](/crates/oxide-sketch/src/geom/triangulate/triangle_emits_one_triangle.md) |
