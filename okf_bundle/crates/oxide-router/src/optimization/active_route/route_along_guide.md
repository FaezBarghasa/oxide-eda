---
okf_version: "0.2"
type: Function
title: route_along_guide
description: Route multiple nets in parallel along a guide curve (River Routing)
resource: crates/oxide-router/src/optimization/active_route.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/optimization/active_route/route_along_guide
language: rust
---

# route_along_guide

Route multiple nets in parallel along a guide curve (River Routing)

## Signature

```rust
impl ActiveRouteOptimizer { pub fn route_along_guide(&self, nets: &[NetId], guide: &RouteGuide) -> Vec<RoutingResult> }
```

## Visibility

- `pub`

## Docstring

Route multiple nets in parallel along a guide curve (River Routing)

## Source
Lines 38–90 in `crates/oxide-router/src/optimization/active_route.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_route](/crates/oxide-router/src/optimization/active_route.md) |
