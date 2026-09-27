---
okf_version: "0.2"
type: Function
title: route_board
description: Route board nets using two-stage (topological + detailed) routing
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/route_board
language: rust
---

# route_board

Route board nets using two-stage (topological + detailed) routing

## Signature

```rust
impl TopologicalAutorouter { pub fn route_board(&mut self, _board: &mut PcbBoard, nets: &[NetId]) -> Vec<RoutingResult> }
```

## Visibility

- `pub`

## Docstring

Route board nets using two-stage (topological + detailed) routing

## Source
Lines 304–313 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
