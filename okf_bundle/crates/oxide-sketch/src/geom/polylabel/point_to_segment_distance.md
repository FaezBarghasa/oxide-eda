---
okf_version: "0.2"
type: Function
title: point_to_segment_distance
resource: crates/oxide-sketch/src/geom/polylabel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/polylabel/point_to_segment_distance
language: rust
---

# point_to_segment_distance

## Signature

```rust
fn point_to_segment_distance(p: Point2, a: Point2, b: Point2) -> f64
```

## Source
Lines 212–224 in `crates/oxide-sketch/src/geom/polylabel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polylabel](/crates/oxide-sketch/src/geom/polylabel.md) |
| called_by | [signed_distance_to_polygon](/crates/oxide-sketch/src/geom/polylabel/signed_distance_to_polygon.md) |
