---
okf_version: "0.2"
type: Function
title: has_edge
resource: crates/oxide-router/src/topology/triangulation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/triangulation/has_edge
language: rust
---

# has_edge

## Signature

```rust
fn has_edge(tri: &[Point2D; 3], edge: [Point2D; 2]) -> bool
```

## Source
Lines 121–130 in `crates/oxide-router/src/topology/triangulation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulation](/crates/oxide-router/src/topology/triangulation.md) |
| called_by | [bowyer_watson_delaunay](/crates/oxide-router/src/topology/triangulation/bowyer_watson_delaunay.md) |
