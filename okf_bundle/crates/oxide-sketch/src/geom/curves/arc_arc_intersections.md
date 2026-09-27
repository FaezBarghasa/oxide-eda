---
okf_version: "0.2"
type: Function
title: arc_arc_intersections
description: Find intersection points of two arcs. Wraps the circle solver
resource: crates/oxide-sketch/src/geom/curves.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/curves/arc_arc_intersections
language: rust
---

# arc_arc_intersections

Find intersection points of two arcs. Wraps the circle solver

## Signature

```rust
pub fn arc_arc_intersections(a: Arc2, b: Arc2) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Find intersection points of two arcs. Wraps the circle solver
and filters BOTH arcs' angular ranges.

## Source
Lines 71–82 in `crates/oxide-sketch/src/geom/curves.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [curves](/crates/oxide-sketch/src/geom/curves.md) |
| calls | [circle_circle_intersections](/crates/oxide-sketch/src/geom/curves/circle_circle_intersections.md) |
| called_by | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
| called_by | [arc_arc_both_sweeps_contain_hit](/crates/oxide-sketch/src/geom/curves/arc_arc_both_sweeps_contain_hit.md) |
| called_by | [arc_arc_no_overlap_in_sweeps](/crates/oxide-sketch/src/geom/curves/arc_arc_no_overlap_in_sweeps.md) |
