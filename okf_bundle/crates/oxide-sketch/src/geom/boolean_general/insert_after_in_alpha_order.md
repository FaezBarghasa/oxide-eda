---
okf_version: "0.2"
type: Function
title: insert_after_in_alpha_order
description: Insert intersection vertex at the right edge-sorted position
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/insert_after_in_alpha_order
language: rust
---

# insert_after_in_alpha_order

Insert intersection vertex at the right edge-sorted position

## Signature

```rust
fn insert_after_in_alpha_order(
    verts: &mut Vec<Vertex>,
    start_idx: usize,
    new_vertex: Vertex,
) -> usize
```

## Docstring

Insert intersection vertex at the right edge-sorted position
between `start_idx` (the corner that begins the edge) and
whatever vertex currently follows it. Multiple intersections
on one edge sort by ascending `alpha`.

## Source
Lines 114–144 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| called_by | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
