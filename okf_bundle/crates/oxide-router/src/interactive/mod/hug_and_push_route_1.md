---
okf_version: "0.2"
type: Function
title: hug_and_push_route
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/hug_and_push_route_1
language: rust
---

# hug_and_push_route

## Signature

```rust
fn hug_and_push_route(
        &self,
        start: Point2D,
        end: Point2D,
        net_id: NetId,
        layer: LayerId,
        width: Microns,
    ) -> Vec<RouteSegment>
```

## Source
Lines 310–319 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
