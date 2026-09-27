---
okf_version: "0.2"
type: Function
title: signed_distance_to_polygon
description: "Signed distance from `p` to the polygon — positive inside,"
resource: crates/oxide-sketch/src/geom/polylabel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/polylabel/signed_distance_to_polygon
language: rust
---

# signed_distance_to_polygon

Signed distance from `p` to the polygon — positive inside,

## Signature

```rust
fn signed_distance_to_polygon(p: Point2, polygon: &[Point2]) -> f64
```

## Docstring

Signed distance from `p` to the polygon — positive inside,
negative outside, zero on the boundary. Magnitude equals the
shortest distance to any polygon edge.

## Source
Lines 174–187 in `crates/oxide-sketch/src/geom/polylabel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polylabel](/crates/oxide-sketch/src/geom/polylabel.md) |
| calls | [point_in_polygon](/crates/oxide-sketch/src/geom/polylabel/point_in_polygon.md) |
| calls | [point_to_segment_distance](/crates/oxide-sketch/src/geom/polylabel/point_to_segment_distance.md) |
| called_by | [l_shape_pole_in_thicker_arm](/crates/oxide-sketch/src/geom/polylabel/l_shape_pole_in_thicker_arm.md) |
| called_by | [new](/crates/oxide-sketch/src/geom/polylabel/new.md) |
