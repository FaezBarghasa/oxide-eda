---
okf_version: "0.2"
type: Function
title: start_routing
description: Start a routing session from a given pad/point.
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/start_routing_1
language: rust
---

# start_routing

Start a routing session from a given pad/point.

## Signature

```rust
pub fn start_routing(
        &mut self,
        start_point: Point2D,
        net_id: NetId,
        layer: LayerId,
    ) -> Result<(), RoutingError>
```

## Visibility

- `pub`

## Docstring

Start a routing session from a given pad/point.

## Source
Lines 54–73 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
