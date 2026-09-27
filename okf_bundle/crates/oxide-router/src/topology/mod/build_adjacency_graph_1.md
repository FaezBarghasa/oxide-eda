---
okf_version: "0.2"
type: Function
title: build_adjacency_graph
description: Build adjacency graph connecting obstacles
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/build_adjacency_graph_1
language: rust
---

# build_adjacency_graph

Build adjacency graph connecting obstacles

## Signature

```rust
fn build_adjacency_graph(
        &self,
        obstacles: &[Obstacle],
        _triangulation: &Triangulation,
    ) -> AdjacencyGraph
```

## Docstring

Build adjacency graph connecting obstacles

## Source
Lines 219–259 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
