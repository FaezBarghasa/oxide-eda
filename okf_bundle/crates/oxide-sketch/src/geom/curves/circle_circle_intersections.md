---
okf_version: "0.2"
type: Function
title: circle_circle_intersections
description: "Find the intersection points of two circles. Returns 0, 1, or"
resource: crates/oxide-sketch/src/geom/curves.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/curves/circle_circle_intersections
language: rust
---

# circle_circle_intersections

Find the intersection points of two circles. Returns 0, 1, or

## Signature

```rust
pub fn circle_circle_intersections(a: Circle2, b: Circle2) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Find the intersection points of two circles. Returns 0, 1, or
2 hit points.

Algorithm — standard:
d = |c1 - c0|
- If d > r0 + r1 → disjoint (0 hits).
- If d < |r0 - r1| → one circle inside the other (0 hits).
- Else two roots at the perpendicular foot:
a = (r0² - r1² + d²) / (2 d)
h = sqrt(r0² - a²)
midpoint = c0 + a * (c1 - c0) / d
hits = midpoint ± h * perpendicular(c1 - c0) / d
- Tangent (d == r0 + r1 or d == |r0 - r1|) collapses to one
hit at the tangency point.

## Source
Lines 24–53 in `crates/oxide-sketch/src/geom/curves.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [curves](/crates/oxide-sketch/src/geom/curves.md) |
| called_by | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
| called_by | [arc_arc_intersections](/crates/oxide-sketch/src/geom/curves/arc_arc_intersections.md) |
| called_by | [arc_circle_intersections](/crates/oxide-sketch/src/geom/curves/arc_circle_intersections.md) |
| called_by | [circles_tangent_single_point](/crates/oxide-sketch/src/geom/curves/circles_tangent_single_point.md) |
| called_by | [circles_two_intersection](/crates/oxide-sketch/src/geom/curves/circles_two_intersection.md) |
