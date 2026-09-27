---
okf_version: "0.2"
type: Function
title: point_to_segment_distance
description: "Euclidean distance from `p` to the segment `[a, b]`."
resource: crates/oxide-sketch/src/geom/point.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/point/point_to_segment_distance
language: rust
---

# point_to_segment_distance

Euclidean distance from `p` to the segment `[a, b]`.

## Signature

```rust
pub fn point_to_segment_distance(
    p: impl Into<Point2>,
    a: impl Into<Point2>,
    b: impl Into<Point2>,
) -> f64
```

## Visibility

- `pub`

## Docstring

Euclidean distance from `p` to the segment `[a, b]`.

## Source
Lines 66–72 in `crates/oxide-sketch/src/geom/point.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point](/crates/oxide-sketch/src/geom/point.md) |
| calls | [point_to_segment_distance_sq](/crates/oxide-sketch/src/geom/point/point_to_segment_distance_sq.md) |
