---
okf_version: "0.2"
type: Function
title: vertex_outgoing
description: "Half-edges originating at a vertex. Walks via `twin.next`"
resource: crates/oxide-sketch/src/geom/halfedge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/geom/halfedge/vertex_outgoing
language: rust
---

# vertex_outgoing

Half-edges originating at a vertex. Walks via `twin.next`

## Signature

```rust
impl Mesh { pub fn vertex_outgoing(&self, v: VertexId) -> Vec<HalfEdgeId> }
```

## Visibility

- `pub`

## Docstring

Half-edges originating at a vertex. Walks via `twin.next`
which advances around the vertex's incident half-edges.

## Source
Lines 182–202 in `crates/oxide-sketch/src/geom/halfedge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [halfedge](/crates/oxide-sketch/src/geom/halfedge.md) |
