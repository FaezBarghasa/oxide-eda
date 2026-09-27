---
okf_version: "0.2"
type: Function
title: identify_routing_channels
description: Identify routing channels between obstacles
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/identify_routing_channels_1
language: rust
---

# identify_routing_channels

Identify routing channels between obstacles

## Signature

```rust
fn identify_routing_channels(
        &self,
        obstacles: &[Obstacle],
        graph: &AdjacencyGraph,
    ) -> Vec<RoutingChannel>
```

## Docstring

Identify routing channels between obstacles

## Source
Lines 262–301 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
