---
okf_version: "0.2"
type: Function
title: differential_pair_route
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/differential_pair_route_1
language: rust
---

# differential_pair_route

## Signature

```rust
fn differential_pair_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment>
```

## Source
Lines 321–362 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
| calls | [find_astar_path](/crates/oxide-router/src/interactive/astar/find_astar_path.md) |
