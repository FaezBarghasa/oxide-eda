---
okf_version: "0.2"
type: Function
title: segment_arc_intersections
description: Intersect a segment with an arc. Built on top of
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/segment_arc_intersections
language: rust
---

# segment_arc_intersections

Intersect a segment with an arc. Built on top of

## Signature

```rust
pub fn segment_arc_intersections(seg: Segment2, arc: Arc2) -> Vec<(Point2, f64)>
```

## Visibility

- `pub`

## Docstring

Intersect a segment with an arc. Built on top of
`segment_circle_intersections` with an angular containment filter
so only points lying within the arc's sweep are returned.

## Source
Lines 272–281 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
| calls | [segment_circle_intersections](/crates/oxide-sketch/src/geom/segment/segment_circle_intersections.md) |
| called_by | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
| called_by | [segment_arc_filters_outside_sweep](/crates/oxide-sketch/src/geom/segment/segment_arc_filters_outside_sweep.md) |
