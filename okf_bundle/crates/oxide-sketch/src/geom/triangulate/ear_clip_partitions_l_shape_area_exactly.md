---
okf_version: "0.2"
type: Function
title: ear_clip_partitions_l_shape_area_exactly
description: "Regression: on this exact L-shape, the first ear-clip (removing"
resource: crates/oxide-sketch/src/geom/triangulate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/triangulate/ear_clip_partitions_l_shape_area_exactly
language: rust
---

# ear_clip_partitions_l_shape_area_exactly

Regression: on this exact L-shape, the first ear-clip (removing

## Signature

```rust
fn ear_clip_partitions_l_shape_area_exactly()
```

## Decorators

- `test`

## Docstring

Regression: on this exact L-shape, the first ear-clip (removing
vertex 0) creates a bridge edge from vertex 5 to vertex 1 that passes
exactly through vertex 3 — `(0,2)` to `(2,0)` is the line `x+y=2`,
and vertex 3 is `(1,1)`. A point-in-triangle test that treats
boundary hits as "outside" for vertices other than the ear's own
corners approves the next ear (removing vertex 1) anyway, because
vertex 3 is exactly ON that bridge edge rather than strictly inside
the candidate triangle. The result: the remaining ring collapses to
zero net area (self-intersecting) instead of a simple quad, so two
of the emitted "triangles" overlap with opposite winding rather than
partitioning the polygon. Every triangle's contribution to the
total area must sum to exactly the polygon's own area — the bug
this pins produced 4.0 instead of the correct 3.0.
[test]

## Source
Lines 354–378 in `crates/oxide-sketch/src/geom/triangulate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulate](/crates/oxide-sketch/src/geom/triangulate.md) |
| calls | [p](/crates/oxide-sketch/src/geom/triangulate/p.md) |
| calls | [ear_clip](/crates/oxide-sketch/src/geom/triangulate/ear_clip.md) |
