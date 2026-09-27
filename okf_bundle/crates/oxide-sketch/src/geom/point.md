---
okf_version: "0.2"
type: Module
title: point
description: Point / segment / polygon predicates shared across the editor surfaces.
resource: crates/oxide-sketch/src/geom/point.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/point
language: rust
---

# point

Point / segment / polygon predicates shared across the editor surfaces.

## Docstring

Point / segment / polygon predicates shared across the editor surfaces.

These were reimplemented (with drifting signatures) in the footprint,
schematic, and symbol presentation code; consolidating them here keeps
one correct, tested implementation in the domain and lets the surfaces
call it through their local point representations.

## Relationships

| Type | Target |
|------|--------|
| related | [point_in_polygon](/crates/oxide-sketch/src/geom/point/point_in_polygon.md) |
| related | [point_to_segment_distance_sq](/crates/oxide-sketch/src/geom/point/point_to_segment_distance_sq.md) |
| related | [point_to_segment_distance](/crates/oxide-sketch/src/geom/point/point_to_segment_distance.md) |
| related | [inside_and_outside_a_square](/crates/oxide-sketch/src/geom/point/inside_and_outside_a_square.md) |
| related | [degenerate_polygon_is_never_inside](/crates/oxide-sketch/src/geom/point/degenerate_polygon_is_never_inside.md) |
| related | [distance_to_segment](/crates/oxide-sketch/src/geom/point/distance_to_segment.md) |
