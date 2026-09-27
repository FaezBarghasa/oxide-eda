---
okf_version: "0.2"
type: Function
title: convex_hull
description: "Build the convex hull of `points`. Returns the hull vertices in"
resource: crates/oxide-sketch/src/geom/hull.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/hull/convex_hull
language: rust
---

# convex_hull

Build the convex hull of `points`. Returns the hull vertices in

## Signature

```rust
pub fn convex_hull(points: &[Point2]) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Build the convex hull of `points`. Returns the hull vertices in
counter-clockwise order. An input with fewer than 3 distinct
points returns the deduplicated input as-is (a zero-area hull).

## Source
Lines 23–76 in `crates/oxide-sketch/src/geom/hull.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hull](/crates/oxide-sketch/src/geom/hull.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
| called_by | [colinear_points_along_edge_are_dropped](/crates/oxide-sketch/src/geom/hull/colinear_points_along_edge_are_dropped.md) |
| called_by | [duplicate_points_are_deduplicated](/crates/oxide-sketch/src/geom/hull/duplicate_points_are_deduplicated.md) |
| called_by | [hexagon_round_trip](/crates/oxide-sketch/src/geom/hull/hexagon_round_trip.md) |
| called_by | [interior_point_is_excluded](/crates/oxide-sketch/src/geom/hull/interior_point_is_excluded.md) |
| called_by | [single_point_returns_itself](/crates/oxide-sketch/src/geom/hull/single_point_returns_itself.md) |
| called_by | [square_corners_are_the_hull](/crates/oxide-sketch/src/geom/hull/square_corners_are_the_hull.md) |
| called_by | [two_points_return_two_points](/crates/oxide-sketch/src/geom/hull/two_points_return_two_points.md) |
