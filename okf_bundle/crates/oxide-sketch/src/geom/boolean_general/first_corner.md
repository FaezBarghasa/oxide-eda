---
okf_version: "0.2"
type: Function
title: first_corner
description: "Find the first non-intersection vertex starting from `start`."
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/first_corner
language: rust
---

# first_corner

Find the first non-intersection vertex starting from `start`.

## Signature

```rust
fn first_corner(verts: &[Vertex], start: usize) -> Option<usize>
```

## Docstring

Find the first non-intersection vertex starting from `start`.
Required for the entry/exit classifier seed: we need a vertex
from the original polygon (not an inserted intersection) so
we can ask "is this point inside the other polygon?" without
the intersection point sitting on its boundary.

## Source
Lines 177–188 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| called_by | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
