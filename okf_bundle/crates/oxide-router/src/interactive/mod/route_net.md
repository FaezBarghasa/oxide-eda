---
okf_version: "0.2"
type: Function
title: route_net
description: Route a complete net from start to target.
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/route_net
language: rust
---

# route_net

Route a complete net from start to target.

## Signature

```rust
impl InteractiveRouter { pub fn route_net(&mut self, net_id: NetId, start: Point2D, target: Point2D) -> RoutingResult }
```

## Visibility

- `pub`

## Docstring

Route a complete net from start to target.

## Source
Lines 194–206 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
