---
okf_version: "0.2"
type: Function
title: line_edge_intersection
description: "Intersection of the segment `(p1, p2)` with the LINE through"
resource: crates/oxide-sketch/src/geom/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean/line_edge_intersection
language: rust
---

# line_edge_intersection

Intersection of the segment `(p1, p2)` with the LINE through

## Signature

```rust
fn line_edge_intersection(p1: Point2, p2: Point2, a: Point2, b: Point2) -> Option<Point2>
```

## Docstring

Intersection of the segment `(p1, p2)` with the LINE through
`(a, b)` (extended infinitely, not the segment). Sutherland-
Hodgman crosses the half-plane edge at the parametric point
where the cross-product flips sign; the intersection always
lies on the segment iff the prev/curr inside flags differ.

## Source
Lines 112–123 in `crates/oxide-sketch/src/geom/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-sketch/src/geom/boolean.md) |
| called_by | [clip_against_edge](/crates/oxide-sketch/src/geom/boolean/clip_against_edge.md) |
