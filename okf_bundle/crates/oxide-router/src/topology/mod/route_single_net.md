---
okf_version: "0.2"
type: Function
title: route_single_net
description: Route a single net
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/route_single_net
language: rust
---

# route_single_net

Route a single net

## Signature

```rust
impl TopologicalAutorouter { pub fn route_single_net(&self, net_id: NetId) -> RoutingResult }
```

## Visibility

- `pub`

## Docstring

Route a single net

## Source
Lines 316–389 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
