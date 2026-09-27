---
okf_version: "0.2"
type: Module
title: simplify
description: "Polygon simplification — removes duplicate vertices, merges"
resource: crates/oxide-sketch/src/geom/simplify.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/simplify
language: rust
---

# simplify

Polygon simplification — removes duplicate vertices, merges

## Docstring

Polygon simplification — removes duplicate vertices, merges
colinear edges, and snaps near-equal coordinates to a grid.
Used as a pre-processing pass before the boolean operations
to dodge the degenerate cases (vertex-on-edge, colinear
shared boundaries, near-duplicate vertices) that trip the
basic Greiner-Hormann variant.

For a future full robust-boolean implementation these helpers
become inner steps of the boolean itself; for the current
Greiner-Hormann they're a "make my inputs cleaner" entry point.

## Relationships

| Type | Target |
|------|--------|
| related | [MultiContour](/crates/oxide-sketch/src/geom/simplify/MultiContour.md) |
| related | [new](/crates/oxide-sketch/src/geom/simplify/new.md) |
| related | [with_holes](/crates/oxide-sketch/src/geom/simplify/with_holes.md) |
| related | [new](/crates/oxide-sketch/src/geom/simplify/new.md) |
| related | [with_holes](/crates/oxide-sketch/src/geom/simplify/with_holes.md) |
| related | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
| related | [merge_colinear](/crates/oxide-sketch/src/geom/simplify/merge_colinear.md) |
| related | [snap_to_grid](/crates/oxide-sketch/src/geom/simplify/snap_to_grid.md) |
| related | [simplify_polygon](/crates/oxide-sketch/src/geom/simplify/simplify_polygon.md) |
| related | [p](/crates/oxide-sketch/src/geom/simplify/p.md) |
| related | [dedup_removes_adjacent_duplicates](/crates/oxide-sketch/src/geom/simplify/dedup_removes_adjacent_duplicates.md) |
| related | [dedup_removes_wrap_around_duplicate](/crates/oxide-sketch/src/geom/simplify/dedup_removes_wrap_around_duplicate.md) |
| related | [merge_colinear_drops_midpoints](/crates/oxide-sketch/src/geom/simplify/merge_colinear_drops_midpoints.md) |
| related | [snap_to_grid_rounds_to_step](/crates/oxide-sketch/src/geom/simplify/snap_to_grid_rounds_to_step.md) |
| related | [simplify_pipeline_combines_all_three](/crates/oxide-sketch/src/geom/simplify/simplify_pipeline_combines_all_three.md) |
