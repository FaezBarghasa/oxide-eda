---
okf_version: "0.2"
type: Module
title: aabb_index
description: Bounding-box spatial index for 2D primitives.
resource: crates/oxide-sketch/src/geom/aabb_index.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/aabb_index
language: rust
---

# aabb_index

Bounding-box spatial index for 2D primitives.

## Docstring

Bounding-box spatial index for 2D primitives.

A flat AABB array with linear-scan queries that uses early
rejection on the bounding box. Backs the snap + hit-test
pipelines once entity counts climb past the point where O(n)
scans become noticeable (~hundreds).

The structure is intentionally simple — no kd-tree, no
R-tree — because:
1. Sketch-mode entity counts are typically <500. The
constant factor of a tree is bigger than the win at this
scale.
2. Inserts / rebuilds happen on every solve+bake. A tree
that pays for fast queries via slow inserts trades the
wrong direction.
3. The flat-array form is trivial to verify and benchmark.

When sketches start carrying tens of thousands of primitives
(PCB-level use of this module), drop a kd-tree behind the same
`AabbIndex` API and the call sites won't change.

## Relationships

| Type | Target |
|------|--------|
| related | [Aabb](/crates/oxide-sketch/src/geom/aabb_index/Aabb.md) |
| related | [new](/crates/oxide-sketch/src/geom/aabb_index/new.md) |
| related | [from_points](/crates/oxide-sketch/src/geom/aabb_index/from_points.md) |
| related | [contains](/crates/oxide-sketch/src/geom/aabb_index/contains.md) |
| related | [overlaps](/crates/oxide-sketch/src/geom/aabb_index/overlaps.md) |
| related | [expanded](/crates/oxide-sketch/src/geom/aabb_index/expanded.md) |
| related | [new](/crates/oxide-sketch/src/geom/aabb_index/new.md) |
| related | [from_points](/crates/oxide-sketch/src/geom/aabb_index/from_points.md) |
| related | [contains](/crates/oxide-sketch/src/geom/aabb_index/contains.md) |
| related | [overlaps](/crates/oxide-sketch/src/geom/aabb_index/overlaps.md) |
| related | [expanded](/crates/oxide-sketch/src/geom/aabb_index/expanded.md) |
| related | [AabbIndex](/crates/oxide-sketch/src/geom/aabb_index/AabbIndex.md) |
| related | [new](/crates/oxide-sketch/src/geom/aabb_index/new.md) |
| related | [with_capacity](/crates/oxide-sketch/src/geom/aabb_index/with_capacity.md) |
| related | [insert](/crates/oxide-sketch/src/geom/aabb_index/insert.md) |
| related | [len](/crates/oxide-sketch/src/geom/aabb_index/len.md) |
| related | [is_empty](/crates/oxide-sketch/src/geom/aabb_index/is_empty.md) |
| related | [clear](/crates/oxide-sketch/src/geom/aabb_index/clear.md) |
| related | [query_point](/crates/oxide-sketch/src/geom/aabb_index/query_point.md) |
| related | [query_region](/crates/oxide-sketch/src/geom/aabb_index/query_region.md) |
| related | [new](/crates/oxide-sketch/src/geom/aabb_index/new.md) |
| related | [with_capacity](/crates/oxide-sketch/src/geom/aabb_index/with_capacity.md) |
| related | [insert](/crates/oxide-sketch/src/geom/aabb_index/insert.md) |
| related | [len](/crates/oxide-sketch/src/geom/aabb_index/len.md) |
| related | [is_empty](/crates/oxide-sketch/src/geom/aabb_index/is_empty.md) |
| related | [clear](/crates/oxide-sketch/src/geom/aabb_index/clear.md) |
| related | [query_point](/crates/oxide-sketch/src/geom/aabb_index/query_point.md) |
| related | [query_region](/crates/oxide-sketch/src/geom/aabb_index/query_region.md) |
| related | [p](/crates/oxide-sketch/src/geom/aabb_index/p.md) |
| related | [aabb_from_points_empty](/crates/oxide-sketch/src/geom/aabb_index/aabb_from_points_empty.md) |
| related | [aabb_from_points_finds_extents](/crates/oxide-sketch/src/geom/aabb_index/aabb_from_points_finds_extents.md) |
| related | [aabb_contains_inclusive](/crates/oxide-sketch/src/geom/aabb_index/aabb_contains_inclusive.md) |
| related | [aabb_overlaps_touching_is_true](/crates/oxide-sketch/src/geom/aabb_index/aabb_overlaps_touching_is_true.md) |
| related | [aabb_overlaps_disjoint_is_false](/crates/oxide-sketch/src/geom/aabb_index/aabb_overlaps_disjoint_is_false.md) |
| related | [index_query_point_finds_overlapping](/crates/oxide-sketch/src/geom/aabb_index/index_query_point_finds_overlapping.md) |
| related | [index_query_region_iterator](/crates/oxide-sketch/src/geom/aabb_index/index_query_region_iterator.md) |
| related | [expanded_grows_in_all_directions](/crates/oxide-sketch/src/geom/aabb_index/expanded_grows_in_all_directions.md) |
