---
okf_version: "0.2"
type: Function
title: signed_area
description: "Twice the signed area of a polygon defined by `points` (treated"
resource: crates/oxide-sketch/src/geom/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/predicates/signed_area
language: rust
---

# signed_area

Twice the signed area of a polygon defined by `points` (treated

## Signature

```rust
pub fn signed_area(points: &[Point2]) -> f64
```

## Visibility

- `pub`

## Docstring

Twice the signed area of a polygon defined by `points` (treated
as a closed ring — last vertex connects back to first). The
shoelace formula. Positive = CCW winding, Negative = CW, Zero
= degenerate. Returns `0.0` for fewer than three vertices.

## Source
Lines 72–83 in `crates/oxide-sketch/src/geom/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-sketch/src/geom/predicates.md) |
| called_by | [area](/crates/oxide-sketch/src/geom/boolean/area.md) |
| called_by | [intersect_convex_clip](/crates/oxide-sketch/src/geom/boolean/intersect_convex_clip.md) |
| called_by | [area](/crates/oxide-sketch/src/geom/boolean_general/area.md) |
| called_by | [from_polygon](/crates/oxide-sketch/src/geom/halfedge/from_polygon.md) |
| called_by | [offset_polygon](/crates/oxide-sketch/src/geom/offset/offset_polygon.md) |
| called_by | [signed_area_unit_square_ccw_is_one](/crates/oxide-sketch/src/geom/predicates/signed_area_unit_square_ccw_is_one.md) |
| called_by | [signed_area_unit_square_cw_is_negative_one](/crates/oxide-sketch/src/geom/predicates/signed_area_unit_square_cw_is_negative_one.md) |
| called_by | [ear_clip](/crates/oxide-sketch/src/geom/triangulate/ear_clip.md) |
| called_by | [ear_clip_with_holes](/crates/oxide-sketch/src/geom/triangulate/ear_clip_with_holes.md) |
