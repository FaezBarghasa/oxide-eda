---
okf_version: "0.2"
type: Function
title: bowyer_watson_delaunay
description: Perform 2D Delaunay Triangulation on a set of vertices using Bowyer-Watson.
resource: crates/oxide-router/src/topology/triangulation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/triangulation/bowyer_watson_delaunay
language: rust
---

# bowyer_watson_delaunay

Perform 2D Delaunay Triangulation on a set of vertices using Bowyer-Watson.

## Signature

```rust
pub fn bowyer_watson_delaunay(vertices: &[Point2D]) -> Vec<Triangle>
```

## Visibility

- `pub`

## Docstring

Perform 2D Delaunay Triangulation on a set of vertices using Bowyer-Watson.

## Source
Lines 9–100 in `crates/oxide-router/src/topology/triangulation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulation](/crates/oxide-router/src/topology/triangulation.md) |
| calls | [in_circumcircle](/crates/oxide-router/src/topology/triangulation/in_circumcircle.md) |
| calls | [has_edge](/crates/oxide-router/src/topology/triangulation/has_edge.md) |
| called_by | [triangulate_free_space](/crates/oxide-router/src/topology/mod/triangulate_free_space.md) |
