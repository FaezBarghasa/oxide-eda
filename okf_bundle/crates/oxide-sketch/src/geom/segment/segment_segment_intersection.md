---
okf_version: "0.2"
type: Function
title: segment_segment_intersection
description: "Intersect two line segments. Returns `None` for parallel-disjoint"
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/segment_segment_intersection
language: rust
---

# segment_segment_intersection

Intersect two line segments. Returns `None` for parallel-disjoint

## Signature

```rust
pub fn segment_segment_intersection(p: Segment2, q: Segment2) -> SegmentIntersection
```

## Visibility

- `pub`

## Docstring

Intersect two line segments. Returns `None` for parallel-disjoint
pairs, `Point` for the standard cross-intersection (or a shared
endpoint), and `Overlap` for colinear segments that share more
than a single point.

Algorithm: solve `A + t*(B-A) = C + s*(D-C)` for (t, s). The
determinant of the 2x2 system tells us the lines aren't parallel;
when it's zero we fall through to a colinear-overlap check using
projection onto the longest axis.

## Source
Lines 149–217 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
| called_by | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
| called_by | [colinear_overlap](/crates/oxide-sketch/src/geom/segment/colinear_overlap.md) |
| called_by | [cross_intersection](/crates/oxide-sketch/src/geom/segment/cross_intersection.md) |
| called_by | [t_intersection_at_endpoint](/crates/oxide-sketch/src/geom/segment/t_intersection_at_endpoint.md) |
