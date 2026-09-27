---
okf_version: "0.2"
type: Function
title: append_edge_quad
description: "Emit two triangles for the rectangle centred on edge `a -> b`, `half` units"
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/append_edge_quad
language: rust
---

# append_edge_quad

Emit two triangles for the rectangle centred on edge `a -> b`, `half` units

## Signature

```rust
fn append_edge_quad(
    vertices: &mut Vec<PolygonVertex>,
    a: [f32; 2],
    b: [f32; 2],
    half: f32,
    color: [f32; 4],
)
```

## Docstring

Emit two triangles for the rectangle centred on edge `a -> b`, `half` units
to either side along the edge normal.

## Source
Lines 134–159 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| called_by | [append_stroke](/crates/oxide-gfx/src/pipeline/polygon/append_stroke.md) |
