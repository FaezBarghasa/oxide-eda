---
okf_version: "0.2"
type: Function
title: face_halfedges
description: "Iterate the half-edges along a face's boundary."
resource: crates/oxide-sketch/src/geom/halfedge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/geom/halfedge/face_halfedges_1
language: rust
---

# face_halfedges

Iterate the half-edges along a face's boundary.

## Signature

```rust
pub fn face_halfedges(&self, f: FaceId) -> Vec<HalfEdgeId>
```

## Visibility

- `pub`

## Docstring

Iterate the half-edges along a face's boundary.

## Source
Lines 154–170 in `crates/oxide-sketch/src/geom/halfedge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [halfedge](/crates/oxide-sketch/src/geom/halfedge.md) |
