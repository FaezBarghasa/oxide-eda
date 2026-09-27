---
okf_version: "0.2"
type: Function
title: point_to_segment_dist
description: Distance (world-mm) from a point to a line segment — a thin adapter
resource: crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/geometry/point_to_segment_dist
language: rust
---

# point_to_segment_dist

Distance (world-mm) from a point to a line segment — a thin adapter

## Signature

```rust
pub(super) fn point_to_segment_dist(px: f64, py: f64, ax: f64, ay: f64, bx: f64, by: f64) -> f64
```

## Visibility

- `pub(super)`

## Docstring

Distance (world-mm) from a point to a line segment — a thin adapter
over [`oxide_sketch::geom::point_to_segment_distance`].

## Source
Lines 30–32 in `crates/oxide-app/src/library/editor/footprint/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/canvas/geometry.md) |
| calls | [point_to_segment_distance](/crates/oxide-app/src/schematic_runtime/mod/point_to_segment_distance.md) |
| called_by | [polygon_outline_hit](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/polygon_outline_hit.md) |
| called_by | [silk_f_hit_at](/crates/oxide-app/src/library/editor/footprint/canvas/mod/silk_f_hit_at.md) |
| called_by | [point_to_segment_dist_zero_length](/crates/oxide-app/src/library/editor/footprint/canvas/tests/point_to_segment_dist_zero_length.md) |
