---
okf_version: "0.2"
type: Function
title: arc_circle_intersections
description: Find intersection points of an arc and a circle. Wraps
resource: crates/oxide-sketch/src/geom/curves.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/curves/arc_circle_intersections
language: rust
---

# arc_circle_intersections

Find intersection points of an arc and a circle. Wraps

## Signature

```rust
pub fn arc_circle_intersections(arc: Arc2, circle: Circle2) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Find intersection points of an arc and a circle. Wraps
`circle_circle_intersections` and filters by the arc's angular
range.

## Source
Lines 58–67 in `crates/oxide-sketch/src/geom/curves.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [curves](/crates/oxide-sketch/src/geom/curves.md) |
| calls | [circle_circle_intersections](/crates/oxide-sketch/src/geom/curves/circle_circle_intersections.md) |
| called_by | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
| called_by | [arc_circle_filters_outside_sweep](/crates/oxide-sketch/src/geom/curves/arc_circle_filters_outside_sweep.md) |
