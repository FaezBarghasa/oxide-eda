---
okf_version: "0.2"
type: Function
title: triangulate_free_space
description: Triangulate free space using Delaunay triangulation
resource: crates/oxide-router/src/topology/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/mod/triangulate_free_space_1
language: rust
---

# triangulate_free_space

Triangulate free space using Delaunay triangulation

## Signature

```rust
fn triangulate_free_space(&self, obstacles: &[Obstacle]) -> Triangulation
```

## Docstring

Triangulate free space using Delaunay triangulation

## Source
Lines 191–216 in `crates/oxide-router/src/topology/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-router/src/topology/mod.md) |
| calls | [bowyer_watson_delaunay](/crates/oxide-router/src/topology/triangulation/bowyer_watson_delaunay.md) |
