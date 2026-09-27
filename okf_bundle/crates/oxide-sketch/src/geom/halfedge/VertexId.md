---
okf_version: "0.2"
type: Class
title: VertexId
description: "Index types so signatures read clearly. `usize` underneath but"
resource: crates/oxide-sketch/src/geom/halfedge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/geom/halfedge/VertexId
language: rust
---

# VertexId

Index types so signatures read clearly. `usize` underneath but

## Signature

```rust
pub struct VertexId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Index types so signatures read clearly. `usize` underneath but
distinct so calls like `mesh.next(half_edge)` don't accidentally
take a vertex id.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 28–28 in `crates/oxide-sketch/src/geom/halfedge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [halfedge](/crates/oxide-sketch/src/geom/halfedge.md) |
| called_by | [from_polygon](/crates/oxide-sketch/src/geom/halfedge/from_polygon.md) |
| called_by | [vertex_outgoing_yields_inner_and_outer](/crates/oxide-sketch/src/geom/halfedge/vertex_outgoing_yields_inner_and_outer.md) |
