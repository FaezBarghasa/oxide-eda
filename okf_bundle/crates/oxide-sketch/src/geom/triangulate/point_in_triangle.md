---
okf_version: "0.2"
type: Function
title: point_in_triangle
description: "Whether `p` lies inside the triangle `a,b,c`, OR exactly on one of its"
resource: crates/oxide-sketch/src/geom/triangulate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/triangulate/point_in_triangle
language: rust
---

# point_in_triangle

Whether `p` lies inside the triangle `a,b,c`, OR exactly on one of its

## Signature

```rust
fn point_in_triangle(p: Point2, a: Point2, b: Point2, c: Point2) -> bool
```

## Docstring

Whether `p` lies inside the triangle `a,b,c`, OR exactly on one of its
edges/their infinite extension. Uses three orientation predicates: `p` is
strictly OUTSIDE only when it falls on opposite sides of at least two of
the triangle's edges (one `Positive`, one `Negative`) — anything else
(all one sign, or one/more `Sign::Zero` alongside a single consistent
sign) counts as blocking.

A point exactly on an edge is intentionally NOT treated as "outside"
here (unlike the classic strict-interior test), because when `p` is some
OTHER remaining polygon vertex rather than one of `a`/`b`/`c` themselves,
clipping this ear would create a new bridge edge running exactly through
`p` — collapsing the remaining ring into a self-intersecting polygon
(zero or negative net area) instead of a simple one. The caller already
excludes the ear's own three corners from this check, so every point
tested here is a genuinely different vertex.

## Source
Lines 164–175 in `crates/oxide-sketch/src/geom/triangulate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [triangulate](/crates/oxide-sketch/src/geom/triangulate.md) |
| called_by | [is_ear](/crates/oxide-sketch/src/geom/triangulate/is_ear.md) |
