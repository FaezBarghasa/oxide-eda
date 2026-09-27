---
okf_version: "0.2"
type: Module
title: polylabel
description: Pole of inaccessibility — finds the point inside a polygon
resource: crates/oxide-sketch/src/geom/polylabel.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/polylabel
language: rust
---

# polylabel

Pole of inaccessibility — finds the point inside a polygon

## Docstring

Pole of inaccessibility — finds the point inside a polygon
that is furthest from any polygon edge. Useful for placing pad
designators / labels at the visual centre of irregular shapes.

Algorithm — quadtree subdivision keyed on a best-case bound:

1. Seed a max-priority queue with the polygon's bounding box
cell and its centre.
2. Repeatedly pop the cell whose `distance + half_diagonal` is
largest (the upper bound on any future cell's centre
distance after subdivision).
3. If `distance` is better than the running best, update best.
4. If the upper bound exceeds best by more than the precision,
subdivide the cell into 4 children and push each.
5. Terminate when the queue is empty or the next upper bound
is within `precision` of the running best.

O(n²) worst-case in the polygon vertex count for the
point-to-polygon distance step; the recursion depth is
bounded by `log2(bbox / precision)` so it converges quickly
for typical sketch polygons.

## Relationships

| Type | Target |
|------|--------|
| related | [Cell](/crates/oxide-sketch/src/geom/polylabel/Cell.md) |
| related | [eq](/crates/oxide-sketch/src/geom/polylabel/eq.md) |
| related | [eq](/crates/oxide-sketch/src/geom/polylabel/eq.md) |
| related | [partial_cmp](/crates/oxide-sketch/src/geom/polylabel/partial_cmp.md) |
| related | [partial_cmp](/crates/oxide-sketch/src/geom/polylabel/partial_cmp.md) |
| related | [cmp](/crates/oxide-sketch/src/geom/polylabel/cmp.md) |
| related | [cmp](/crates/oxide-sketch/src/geom/polylabel/cmp.md) |
| related | [new](/crates/oxide-sketch/src/geom/polylabel/new.md) |
| related | [new](/crates/oxide-sketch/src/geom/polylabel/new.md) |
| related | [pole_of_inaccessibility](/crates/oxide-sketch/src/geom/polylabel/pole_of_inaccessibility.md) |
| related | [polygon_centroid](/crates/oxide-sketch/src/geom/polylabel/polygon_centroid.md) |
| related | [signed_distance_to_polygon](/crates/oxide-sketch/src/geom/polylabel/signed_distance_to_polygon.md) |
| related | [point_in_polygon](/crates/oxide-sketch/src/geom/polylabel/point_in_polygon.md) |
| related | [point_to_segment_distance](/crates/oxide-sketch/src/geom/polylabel/point_to_segment_distance.md) |
| related | [p](/crates/oxide-sketch/src/geom/polylabel/p.md) |
| related | [close](/crates/oxide-sketch/src/geom/polylabel/close.md) |
| related | [empty_polygon_returns_none](/crates/oxide-sketch/src/geom/polylabel/empty_polygon_returns_none.md) |
| related | [unit_square_pole_at_centre](/crates/oxide-sketch/src/geom/polylabel/unit_square_pole_at_centre.md) |
| related | [rectangle_pole_at_centre](/crates/oxide-sketch/src/geom/polylabel/rectangle_pole_at_centre.md) |
| related | [l_shape_pole_in_thicker_arm](/crates/oxide-sketch/src/geom/polylabel/l_shape_pole_in_thicker_arm.md) |
