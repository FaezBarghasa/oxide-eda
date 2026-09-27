---
okf_version: "0.2"
type: Function
title: is_ear
description: "`true` when the triangle at index `(prev, curr, next)` is an ear"
resource: crates/oxide-sketch/src/geom/triangulate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/triangulate/is_ear
language: rust
---

# is_ear

`true` when the triangle at index `(prev, curr, next)` is an ear

## Signature

```rust
fn is_ear(polygon: &[Point2], ring: &[usize], prev: usize, curr: usize, next: usize) -> bool
```

## Docstring

`true` when the triangle at index `(prev, curr, next)` is an ear
of the CCW-oriented polygon: convex AND no other polygon vertex
lies inside it. Operates on the CCW-canonicalised view via
`ring`.

## Source
Lines 111–147 in `crates/oxide-sketch/src/geom/triangulate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulate](/crates/oxide-sketch/src/geom/triangulate.md) |
| calls | [point_in_triangle](/crates/oxide-sketch/src/geom/triangulate/point_in_triangle.md) |
| called_by | [ear_clip](/crates/oxide-sketch/src/geom/triangulate/ear_clip.md) |
