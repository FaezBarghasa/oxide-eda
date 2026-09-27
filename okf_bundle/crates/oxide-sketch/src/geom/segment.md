---
okf_version: "0.2"
type: Module
title: segment
description: "Segment, circle, and arc intersection helpers."
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment
language: rust
---

# segment

Segment, circle, and arc intersection helpers.

## Docstring

Segment, circle, and arc intersection helpers.

All inputs are in plane-local mm. Returned intersections include
the parameter `t ∈ [0, 1]` along the segment so callers can
reconstruct the world-mm hit position via `seg.at(t)` and order
multiple intersections along the ray direction.

## Relationships

| Type | Target |
|------|--------|
| related | [Segment2](/crates/oxide-sketch/src/geom/segment/Segment2.md) |
| related | [new](/crates/oxide-sketch/src/geom/segment/new.md) |
| related | [at](/crates/oxide-sketch/src/geom/segment/at.md) |
| related | [dx](/crates/oxide-sketch/src/geom/segment/dx.md) |
| related | [dy](/crates/oxide-sketch/src/geom/segment/dy.md) |
| related | [length_sq](/crates/oxide-sketch/src/geom/segment/length_sq.md) |
| related | [new](/crates/oxide-sketch/src/geom/segment/new.md) |
| related | [at](/crates/oxide-sketch/src/geom/segment/at.md) |
| related | [dx](/crates/oxide-sketch/src/geom/segment/dx.md) |
| related | [dy](/crates/oxide-sketch/src/geom/segment/dy.md) |
| related | [length_sq](/crates/oxide-sketch/src/geom/segment/length_sq.md) |
| related | [Circle2](/crates/oxide-sketch/src/geom/segment/Circle2.md) |
| related | [new](/crates/oxide-sketch/src/geom/segment/new.md) |
| related | [new](/crates/oxide-sketch/src/geom/segment/new.md) |
| related | [Arc2](/crates/oxide-sketch/src/geom/segment/Arc2.md) |
| related | [new](/crates/oxide-sketch/src/geom/segment/new.md) |
| related | [contains_angle](/crates/oxide-sketch/src/geom/segment/contains_angle.md) |
| related | [new](/crates/oxide-sketch/src/geom/segment/new.md) |
| related | [contains_angle](/crates/oxide-sketch/src/geom/segment/contains_angle.md) |
| related | [SegmentIntersection](/crates/oxide-sketch/src/geom/segment/SegmentIntersection.md) |
| related | [segment_segment_intersection](/crates/oxide-sketch/src/geom/segment/segment_segment_intersection.md) |
| related | [segment_circle_intersections](/crates/oxide-sketch/src/geom/segment/segment_circle_intersections.md) |
| related | [segment_arc_intersections](/crates/oxide-sketch/src/geom/segment/segment_arc_intersections.md) |
| related | [left_turn_or_colinear](/crates/oxide-sketch/src/geom/segment/left_turn_or_colinear.md) |
| related | [p](/crates/oxide-sketch/src/geom/segment/p.md) |
| related | [close](/crates/oxide-sketch/src/geom/segment/close.md) |
| related | [cross_intersection](/crates/oxide-sketch/src/geom/segment/cross_intersection.md) |
| related | [parallel_disjoint](/crates/oxide-sketch/src/geom/segment/parallel_disjoint.md) |
| related | [colinear_overlap](/crates/oxide-sketch/src/geom/segment/colinear_overlap.md) |
| related | [t_intersection_at_endpoint](/crates/oxide-sketch/src/geom/segment/t_intersection_at_endpoint.md) |
| related | [miss_outside_segment_range](/crates/oxide-sketch/src/geom/segment/miss_outside_segment_range.md) |
| related | [segment_circle_two_hits](/crates/oxide-sketch/src/geom/segment/segment_circle_two_hits.md) |
| related | [segment_circle_tangent_one_hit](/crates/oxide-sketch/src/geom/segment/segment_circle_tangent_one_hit.md) |
| related | [segment_circle_miss](/crates/oxide-sketch/src/geom/segment/segment_circle_miss.md) |
| related | [segment_circle_partial_inside_segment](/crates/oxide-sketch/src/geom/segment/segment_circle_partial_inside_segment.md) |
| related | [arc_angle_containment_within_quadrant](/crates/oxide-sketch/src/geom/segment/arc_angle_containment_within_quadrant.md) |
| related | [arc_seam_crossing](/crates/oxide-sketch/src/geom/segment/arc_seam_crossing.md) |
| related | [segment_arc_filters_outside_sweep](/crates/oxide-sketch/src/geom/segment/segment_arc_filters_outside_sweep.md) |
