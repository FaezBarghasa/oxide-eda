---
okf_version: "0.2"
type: Function
title: face_vertices
description: Vertices around a face in boundary order.
resource: crates/oxide-sketch/src/geom/halfedge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/geom/halfedge/face_vertices_1
language: rust
---

# face_vertices

Vertices around a face in boundary order.

## Signature

```rust
pub fn face_vertices(&self, f: FaceId) -> Vec<VertexId>
```

## Visibility

- `pub`

## Docstring

Vertices around a face in boundary order.

## Source
Lines 173–178 in `crates/oxide-sketch/src/geom/halfedge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [halfedge](/crates/oxide-sketch/src/geom/halfedge.md) |
