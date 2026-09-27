---
okf_version: "0.2"
type: Function
title: find_astar_path_with_ml
description: "Find a clear path using ML-guided A* heuristics while maintaining 100% deterministic DRC enforcement."
resource: crates/oxide-router/src/interactive/astar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:19:36Z"
concept_id: crates/oxide-router/src/interactive/astar/find_astar_path_with_ml
language: rust
---

# find_astar_path_with_ml

Find a clear path using ML-guided A* heuristics while maintaining 100% deterministic DRC enforcement.

## Signature

```rust
pub fn find_astar_path_with_ml(
    spatial_index: &SpatialIndex,
    start: Point2D,
    target: Point2D,
    net_id: NetId,
    width: Microns,
    advisor: Option<&mut oxide_ml::RoutingAdvisor>,
) -> Vec<Point2D>
```

## Visibility

- `pub`

## Docstring

Find a clear path using ML-guided A* heuristics while maintaining 100% deterministic DRC enforcement.

## Source
Lines 124–242 in `crates/oxide-router/src/interactive/astar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [astar](/crates/oxide-router/src/interactive/astar.md) |
| calls | [heuristic](/crates/oxide-router/src/interactive/astar/heuristic.md) |
| calls | [simplify_path](/crates/oxide-router/src/interactive/astar/simplify_path.md) |
| called_by | [test_ml_guided_astar_routing](/crates/oxide-router/tests/router_tests/test_ml_guided_astar_routing.md) |
