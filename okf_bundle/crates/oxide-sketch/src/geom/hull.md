---
okf_version: "0.2"
type: Module
title: hull
description: "Convex hull via the monotone-chain method, O(n log n)."
resource: crates/oxide-sketch/src/geom/hull.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/hull
language: rust
---

# hull

Convex hull via the monotone-chain method, O(n log n).

## Docstring

Convex hull via the monotone-chain method, O(n log n).

Strategy:
1. Sort points lexicographically by `(x, y)`.
2. Build the lower hull walking points left-to-right, popping
the back of the stack while the turn at the new point is a
right turn (i.e. not a left turn).
3. Build the upper hull walking right-to-left with the same
rule.
4. Concatenate, dropping the duplicated end point of each
subhull.

The output is a CCW polygon in standard orientation. Duplicate
and colinear points are handled — colinear points on the hull
edge are dropped (only the extreme endpoints survive).

## Relationships

| Type | Target |
|------|--------|
| related | [convex_hull](/crates/oxide-sketch/src/geom/hull/convex_hull.md) |
| related | [p](/crates/oxide-sketch/src/geom/hull/p.md) |
| related | [empty_input_returns_empty_hull](/crates/oxide-sketch/src/geom/hull/empty_input_returns_empty_hull.md) |
| related | [single_point_returns_itself](/crates/oxide-sketch/src/geom/hull/single_point_returns_itself.md) |
| related | [two_points_return_two_points](/crates/oxide-sketch/src/geom/hull/two_points_return_two_points.md) |
| related | [square_corners_are_the_hull](/crates/oxide-sketch/src/geom/hull/square_corners_are_the_hull.md) |
| related | [interior_point_is_excluded](/crates/oxide-sketch/src/geom/hull/interior_point_is_excluded.md) |
| related | [colinear_points_along_edge_are_dropped](/crates/oxide-sketch/src/geom/hull/colinear_points_along_edge_are_dropped.md) |
| related | [duplicate_points_are_deduplicated](/crates/oxide-sketch/src/geom/hull/duplicate_points_are_deduplicated.md) |
| related | [hexagon_round_trip](/crates/oxide-sketch/src/geom/hull/hexagon_round_trip.md) |
