---
okf_version: "0.2"
type: Function
title: p
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/p
language: rust
---

# p

## Signature

```rust
fn p(x: f64, y: f64) -> Point2
```

## Source
Lines 295–297 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
| called_by | [arc_angle_containment_within_quadrant](/crates/oxide-sketch/src/geom/segment/arc_angle_containment_within_quadrant.md) |
| called_by | [arc_seam_crossing](/crates/oxide-sketch/src/geom/segment/arc_seam_crossing.md) |
| called_by | [colinear_overlap](/crates/oxide-sketch/src/geom/segment/colinear_overlap.md) |
| called_by | [cross_intersection](/crates/oxide-sketch/src/geom/segment/cross_intersection.md) |
| called_by | [miss_outside_segment_range](/crates/oxide-sketch/src/geom/segment/miss_outside_segment_range.md) |
| called_by | [parallel_disjoint](/crates/oxide-sketch/src/geom/segment/parallel_disjoint.md) |
| called_by | [segment_arc_filters_outside_sweep](/crates/oxide-sketch/src/geom/segment/segment_arc_filters_outside_sweep.md) |
| called_by | [segment_circle_miss](/crates/oxide-sketch/src/geom/segment/segment_circle_miss.md) |
| called_by | [segment_circle_partial_inside_segment](/crates/oxide-sketch/src/geom/segment/segment_circle_partial_inside_segment.md) |
| called_by | [segment_circle_tangent_one_hit](/crates/oxide-sketch/src/geom/segment/segment_circle_tangent_one_hit.md) |
| called_by | [segment_circle_two_hits](/crates/oxide-sketch/src/geom/segment/segment_circle_two_hits.md) |
| called_by | [t_intersection_at_endpoint](/crates/oxide-sketch/src/geom/segment/t_intersection_at_endpoint.md) |
