---
okf_version: "0.2"
type: Function
title: point_to_segment_distance_sq
description: "Squared distance from `p` to the segment `[a, b]`, using the clamped"
resource: crates/oxide-sketch/src/geom/point.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/point/point_to_segment_distance_sq
language: rust
---

# point_to_segment_distance_sq

Squared distance from `p` to the segment `[a, b]`, using the clamped

## Signature

```rust
pub fn point_to_segment_distance_sq(
    p: impl Into<Point2>,
    a: impl Into<Point2>,
    b: impl Into<Point2>,
) -> f64
```

## Visibility

- `pub`

## Docstring

Squared distance from `p` to the segment `[a, b]`, using the clamped
projection of `p` onto the segment. Degenerate (zero-length) segments
fall back to the point-to-`a` distance.

## Source
Lines 45–63 in `crates/oxide-sketch/src/geom/point.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [point](/crates/oxide-sketch/src/geom/point.md) |
| called_by | [point_to_segment_dist_sq](/crates/oxide-app/src/library/editor/symbol/state/hit_test/point_to_segment_dist_sq.md) |
| called_by | [point_to_segment_distance](/crates/oxide-sketch/src/geom/point/point_to_segment_distance.md) |
