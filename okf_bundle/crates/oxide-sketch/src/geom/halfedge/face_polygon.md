---
okf_version: "0.2"
type: Function
title: face_polygon
description: "Convenience: positions of the boundary vertices of `f` in"
resource: crates/oxide-sketch/src/geom/halfedge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/geom/halfedge/face_polygon
language: rust
---

# face_polygon

Convenience: positions of the boundary vertices of `f` in

## Signature

```rust
impl Mesh { pub fn face_polygon(&self, f: FaceId) -> Vec<Point2> }
```

## Visibility

- `pub`

## Docstring

Convenience: positions of the boundary vertices of `f` in
boundary order. The classic "extract the polygon outline
from face X" primitive.

## Source
Lines 207–212 in `crates/oxide-sketch/src/geom/halfedge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [halfedge](/crates/oxide-sketch/src/geom/halfedge.md) |
