---
okf_version: "0.2"
type: Function
title: segment_circle_intersections
description: "Intersect a segment with a circle. Returns 0, 1, or 2 hit points"
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/segment_circle_intersections
language: rust
---

# segment_circle_intersections

Intersect a segment with a circle. Returns 0, 1, or 2 hit points

## Signature

```rust
pub fn segment_circle_intersections(seg: Segment2, circle: Circle2) -> Vec<(Point2, f64)>
```

## Visibility

- `pub`

## Docstring

Intersect a segment with a circle. Returns 0, 1, or 2 hit points
with their `t` values along the segment in ascending order.

Algorithm: solve `|A + t*d - C|² = r²` for `t`, a quadratic in
`t` with coefficients
```text
a = d·d
b = 2 * d·(A - C)
c = (A - C)·(A - C) - r²
```
The discriminant `b² - 4ac` discriminates the three cases. Both
roots are filtered to `[0, 1]` so the returned hits lie on the
segment, not the extended line.

## Source
Lines 232–267 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
| called_by | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
| called_by | [segment_arc_intersections](/crates/oxide-sketch/src/geom/segment/segment_arc_intersections.md) |
| called_by | [segment_circle_partial_inside_segment](/crates/oxide-sketch/src/geom/segment/segment_circle_partial_inside_segment.md) |
| called_by | [segment_circle_tangent_one_hit](/crates/oxide-sketch/src/geom/segment/segment_circle_tangent_one_hit.md) |
| called_by | [segment_circle_two_hits](/crates/oxide-sketch/src/geom/segment/segment_circle_two_hits.md) |
