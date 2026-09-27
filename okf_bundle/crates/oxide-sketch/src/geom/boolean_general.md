---
okf_version: "0.2"
type: Module
title: boolean_general
description: "General polygon boolean operations — union, intersection,"
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general
language: rust
---

# boolean_general

General polygon boolean operations — union, intersection,

## Docstring

General polygon boolean operations — union, intersection,
difference, xor — via the Greiner-Hormann clipping pattern.

Handles concave subject polygons against concave clip polygons
and returns a list of result rings (booleans can produce
multiple disjoint output polygons even from connected inputs).

# Limitations

This is the non-degenerate variant: it assumes intersections
between edges occur strictly in their interiors. Vertex-on-edge,
colinear edges, and shared boundaries are NOT handled — those
produce undefined output. Callers needing those cases should
pre-perturb their inputs (offset by a tiny non-uniform jitter)
or fall back to [`super::boolean::intersect_convex_clip`] when
the clip is convex.

# Algorithm overview

1. Build doubly-linked vertex rings for both polygons. Each
vertex carries an `intersect` flag, an `entry` flag, a
`neighbor` link to the matching vertex in the other ring,
and an `alpha` value for sorted insertion.
2. Walk every pair of (subject edge, clip edge), compute
intersections, and insert a paired vertex into both rings
at the right alpha-sorted edge positions.
3. Classify every intersection as "entry" or "exit" relative to
the OTHER polygon's interior. Done by tracking running
inside-state during a single ring walk per polygon, seeded
from a point-in-polygon test on the first non-intersection
vertex.
4. For each unvisited entry intersection, walk the result ring:
advance through the current polygon, switch polygons at the
next intersection, and continue until returning to the start.
The walk direction depends on the operation.

## Relationships

| Type | Target |
|------|--------|
| related | [BoolOp](/crates/oxide-sketch/src/geom/boolean_general/BoolOp.md) |
| related | [Vertex](/crates/oxide-sketch/src/geom/boolean_general/Vertex.md) |
| related | [corner](/crates/oxide-sketch/src/geom/boolean_general/corner.md) |
| related | [intersection](/crates/oxide-sketch/src/geom/boolean_general/intersection.md) |
| related | [corner](/crates/oxide-sketch/src/geom/boolean_general/corner.md) |
| related | [intersection](/crates/oxide-sketch/src/geom/boolean_general/intersection.md) |
| related | [build_ring](/crates/oxide-sketch/src/geom/boolean_general/build_ring.md) |
| related | [insert_after_in_alpha_order](/crates/oxide-sketch/src/geom/boolean_general/insert_after_in_alpha_order.md) |
| related | [point_in_ring](/crates/oxide-sketch/src/geom/boolean_general/point_in_ring.md) |
| related | [first_corner](/crates/oxide-sketch/src/geom/boolean_general/first_corner.md) |
| related | [classify_entries](/crates/oxide-sketch/src/geom/boolean_general/classify_entries.md) |
| related | [is_walk_start](/crates/oxide-sketch/src/geom/boolean_general/is_walk_start.md) |
| related | [walk_one_ring](/crates/oxide-sketch/src/geom/boolean_general/walk_one_ring.md) |
| related | [subject_start_pos_or_default](/crates/oxide-sketch/src/geom/boolean_general/subject_start_pos_or_default.md) |
| related | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
| related | [proper_segment_intersection](/crates/oxide-sketch/src/geom/boolean_general/proper_segment_intersection.md) |
| related | [p](/crates/oxide-sketch/src/geom/boolean_general/p.md) |
| related | [area](/crates/oxide-sketch/src/geom/boolean_general/area.md) |
| related | [disjoint_intersection_is_empty](/crates/oxide-sketch/src/geom/boolean_general/disjoint_intersection_is_empty.md) |
| related | [disjoint_union_is_two_polygons](/crates/oxide-sketch/src/geom/boolean_general/disjoint_union_is_two_polygons.md) |
| related | [disjoint_difference_is_subject](/crates/oxide-sketch/src/geom/boolean_general/disjoint_difference_is_subject.md) |
| related | [quarter_overlap_intersection_unit_area](/crates/oxide-sketch/src/geom/boolean_general/quarter_overlap_intersection_unit_area.md) |
| related | [fully_contained_intersection_returns_inner](/crates/oxide-sketch/src/geom/boolean_general/fully_contained_intersection_returns_inner.md) |
| related | [fully_contained_union_returns_outer](/crates/oxide-sketch/src/geom/boolean_general/fully_contained_union_returns_outer.md) |
