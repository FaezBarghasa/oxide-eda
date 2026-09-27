---
okf_version: "0.2"
type: Function
title: from_polygon
description: Build a mesh from a single closed polygon. The input must
resource: crates/oxide-sketch/src/geom/halfedge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/geom/halfedge/from_polygon_1
language: rust
---

# from_polygon

Build a mesh from a single closed polygon. The input must

## Signature

```rust
pub fn from_polygon(polygon: &[Point2]) -> Option<Self>
```

## Visibility

- `pub`

## Docstring

Build a mesh from a single closed polygon. The input must
have ≥ 3 vertices and no last-vertex-equals-first repeat.
Returns `None` for degenerate input. Output mesh has two
faces — face 0 is the inside (CCW boundary), face 1 is the
unbounded outer (CW boundary).

## Source
Lines 77–139 in `crates/oxide-sketch/src/geom/halfedge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [halfedge](/crates/oxide-sketch/src/geom/halfedge.md) |
| calls | [signed_area](/crates/oxide-sketch/src/geom/predicates/signed_area.md) |
| calls | [HalfEdgeId](/crates/oxide-sketch/src/geom/halfedge/HalfEdgeId.md) |
| calls | [VertexId](/crates/oxide-sketch/src/geom/halfedge/VertexId.md) |
| calls | [FaceId](/crates/oxide-sketch/src/geom/halfedge/FaceId.md) |
