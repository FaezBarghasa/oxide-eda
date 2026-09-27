---
okf_version: "0.2"
type: Function
title: in_circumcircle
description: Test if point is inside circumcircle of triangle.
resource: crates/oxide-router/src/topology/triangulation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/topology/triangulation/in_circumcircle
language: rust
---

# in_circumcircle

Test if point is inside circumcircle of triangle.

## Signature

```rust
fn in_circumcircle(point: &Point2D, triangle: &[Point2D; 3]) -> bool
```

## Docstring

Test if point is inside circumcircle of triangle.

## Source
Lines 103–119 in `crates/oxide-router/src/topology/triangulation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulation](/crates/oxide-router/src/topology/triangulation.md) |
| called_by | [bowyer_watson_delaunay](/crates/oxide-router/src/topology/triangulation/bowyer_watson_delaunay.md) |
