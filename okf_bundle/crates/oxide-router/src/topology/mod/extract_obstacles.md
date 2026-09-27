---
okf_version: "0.2"
type: Function
title: extract_obstacles
description: Extract obstacles from PCB board
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/extract_obstacles
language: rust
---

# extract_obstacles

Extract obstacles from PCB board

## Signature

```rust
impl TopologicalAutorouter { fn extract_obstacles(&self, board: &PcbBoard) -> Vec<Obstacle> }
```

## Docstring

Extract obstacles from PCB board

## Source
Lines 152–188 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
| calls | [Polygon](/crates/oxide-library/src/primitive/footprint/pad/Polygon.md) |
