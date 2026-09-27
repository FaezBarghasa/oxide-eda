---
okf_version: "0.2"
type: Function
title: build_topological_map
description: Build topological map from PCB board
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/build_topological_map_1
language: rust
---

# build_topological_map

Build topological map from PCB board

## Signature

```rust
pub fn build_topological_map(&mut self, board: &PcbBoard) -> Result<(), RoutingError>
```

## Visibility

- `pub`

## Docstring

Build topological map from PCB board

## Source
Lines 135–149 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
