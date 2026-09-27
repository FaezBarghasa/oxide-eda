---
okf_version: "0.2"
type: Function
title: point_in_ring
description: Even-odd point-in-polygon test against the linked ring rooted
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/point_in_ring
language: rust
---

# point_in_ring

Even-odd point-in-polygon test against the linked ring rooted

## Signature

```rust
fn point_in_ring(verts: &[Vertex], head: usize, p: Point2) -> bool
```

## Docstring

Even-odd point-in-polygon test against the linked ring rooted
at `head`. Walks the ring's `next` chain to gather edges.

## Source
Lines 148–170 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| called_by | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
