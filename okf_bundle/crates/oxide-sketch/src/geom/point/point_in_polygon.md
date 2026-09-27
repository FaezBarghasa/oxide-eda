---
okf_version: "0.2"
type: Function
title: point_in_polygon
description: Even-odd ray-cast point-in-polygon test. The polygon is implicitly
resource: crates/oxide-sketch/src/geom/point.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/point/point_in_polygon
language: rust
---

# point_in_polygon

Even-odd ray-cast point-in-polygon test. The polygon is implicitly

## Signature

```rust
pub fn point_in_polygon(p: impl Into<Point2>, polygon: &[Point2]) -> bool
```

## Visibility

- `pub`

## Docstring

Even-odd ray-cast point-in-polygon test. The polygon is implicitly
closed (its last vertex connects back to the first). Near-horizontal
edges (`|dy| < 1e-10`) contribute no crossing — this matches the
standard even-odd rule for horizontal edges and removes a
NaN-propagation path that could otherwise corrupt the toggle for the
remaining edges.

## Source
Lines 16–40 in `crates/oxide-sketch/src/geom/point.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point](/crates/oxide-sketch/src/geom/point.md) |
