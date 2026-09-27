---
okf_version: "0.2"
type: Function
title: plan_path_nodes
description: "A* graph path planning on adjacency graph nodes"
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/plan_path_nodes
language: rust
---

# plan_path_nodes

A* graph path planning on adjacency graph nodes

## Signature

```rust
impl TopologicalAutorouter { pub fn plan_path_nodes(&self, start_id: NodeId, end_id: NodeId) -> Vec<Point2D> }
```

## Visibility

- `pub`

## Docstring

A* graph path planning on adjacency graph nodes

## Source
Lines 392–436 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
