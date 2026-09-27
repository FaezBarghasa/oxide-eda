---
okf_version: "0.2"
type: Function
title: ear_clip_with_holes
description: Triangulate an outer polygon with optional hole rings. Each
resource: crates/oxide-sketch/src/geom/triangulate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/triangulate/ear_clip_with_holes
language: rust
---

# ear_clip_with_holes

Triangulate an outer polygon with optional hole rings. Each

## Signature

```rust
pub fn ear_clip_with_holes(
    outer: &[Point2],
    holes: &[Vec<Point2>],
) -> (Vec<Point2>, Vec<[usize; 3]>)
```

## Visibility

- `pub`

## Docstring

Triangulate an outer polygon with optional hole rings. Each
hole is bridged to the outer ring via a "cut" edge connecting
the rightmost hole vertex to the nearest outer vertex visible
to it; after all holes are bridged the merged ring is a
simple polygon that the basic ear-clipper handles.

Output triangle indices reference a flat vertex array
`[outer_vertices, hole_0_vertices, hole_1_vertices, ...]` in
the same order as the input. Hole rings should wind opposite
to the outer ring (CCW outer + CW holes by convention) but
the implementation normalises before merging.

## Source
Lines 188–242 in `crates/oxide-sketch/src/geom/triangulate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulate](/crates/oxide-sketch/src/geom/triangulate.md) |
| calls | [ear_clip](/crates/oxide-sketch/src/geom/triangulate/ear_clip.md) |
| calls | [signed_area](/crates/oxide-sketch/src/geom/predicates/signed_area.md) |
| calls | [bridge_hole_into_outer](/crates/oxide-sketch/src/geom/triangulate/bridge_hole_into_outer.md) |
| called_by | [ear_clip_with_holes_no_holes_falls_through](/crates/oxide-sketch/src/geom/triangulate/ear_clip_with_holes_no_holes_falls_through.md) |
| called_by | [ear_clip_with_one_hole_emits_more_triangles](/crates/oxide-sketch/src/geom/triangulate/ear_clip_with_one_hole_emits_more_triangles.md) |
