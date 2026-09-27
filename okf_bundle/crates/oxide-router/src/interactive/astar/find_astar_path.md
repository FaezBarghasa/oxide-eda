---
okf_version: "0.2"
type: Function
title: find_astar_path
description: "Find a clear path from `start` to `target` using weighted A* on a routing grid."
resource: crates/oxide-router/src/interactive/astar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:19:36Z"
concept_id: crates/oxide-router/src/interactive/astar/find_astar_path
language: rust
---

# find_astar_path

Find a clear path from `start` to `target` using weighted A* on a routing grid.

## Signature

```rust
pub fn find_astar_path(
    spatial_index: &SpatialIndex,
    start: Point2D,
    target: Point2D,
    net_id: NetId,
    width: Microns,
) -> Vec<Point2D>
```

## Visibility

- `pub`

## Docstring

Find a clear path from `start` to `target` using weighted A* on a routing grid.

## Source
Lines 18–121 in `crates/oxide-router/src/interactive/astar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [astar](/crates/oxide-router/src/interactive/astar.md) |
| calls | [heuristic](/crates/oxide-router/src/interactive/astar/heuristic.md) |
| calls | [simplify_path](/crates/oxide-router/src/interactive/astar/simplify_path.md) |
| called_by | [differential_pair_route](/crates/oxide-router/src/interactive/mod/differential_pair_route.md) |
| called_by | [walk_around_route](/crates/oxide-router/src/interactive/mod/walk_around_route.md) |
| called_by | [retrace_route](/crates/oxide-router/src/optimization/retrace/retrace_route.md) |
