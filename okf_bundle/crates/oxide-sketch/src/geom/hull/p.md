---
okf_version: "0.2"
type: Function
title: p
resource: crates/oxide-sketch/src/geom/hull.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/hull/p
language: rust
---

# p

## Signature

```rust
fn p(x: f64, y: f64) -> Point2
```

## Source
Lines 82–84 in `crates/oxide-sketch/src/geom/hull.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hull](/crates/oxide-sketch/src/geom/hull.md) |
| called_by | [colinear_points_along_edge_are_dropped](/crates/oxide-sketch/src/geom/hull/colinear_points_along_edge_are_dropped.md) |
| called_by | [duplicate_points_are_deduplicated](/crates/oxide-sketch/src/geom/hull/duplicate_points_are_deduplicated.md) |
| called_by | [interior_point_is_excluded](/crates/oxide-sketch/src/geom/hull/interior_point_is_excluded.md) |
| called_by | [single_point_returns_itself](/crates/oxide-sketch/src/geom/hull/single_point_returns_itself.md) |
| called_by | [square_corners_are_the_hull](/crates/oxide-sketch/src/geom/hull/square_corners_are_the_hull.md) |
| called_by | [two_points_return_two_points](/crates/oxide-sketch/src/geom/hull/two_points_return_two_points.md) |
