---
okf_version: "0.2"
type: Module
title: boolean
description: Polygon boolean operations.
resource: crates/oxide-sketch/src/geom/boolean.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean
language: rust
---

# boolean

Polygon boolean operations.

## Docstring

Polygon boolean operations.

Phase 2 stage 1: polygon-against-convex-clip via the
Sutherland-Hodgman recursive-edge-clip algorithm. Handles any
subject polygon (convex or concave) clipped against a convex
clip polygon. Returns the intersection of the two as a single
polygon ring.

General-polygon ↔ general-polygon booleans (Vatti, Greiner-
Hormann) are deferred — the convex-clip case covers viewport
clipping, courtyard clipping, and mask-window cutouts (where
the clip is always a rectangle or a fixed shape) which cover
~80% of the practical use sites in the editor today.

## Relationships

| Type | Target |
|------|--------|
| related | [intersect_convex_clip](/crates/oxide-sketch/src/geom/boolean/intersect_convex_clip.md) |
| related | [clip_against_edge](/crates/oxide-sketch/src/geom/boolean/clip_against_edge.md) |
| related | [is_inside](/crates/oxide-sketch/src/geom/boolean/is_inside.md) |
| related | [line_edge_intersection](/crates/oxide-sketch/src/geom/boolean/line_edge_intersection.md) |
| related | [p](/crates/oxide-sketch/src/geom/boolean/p.md) |
| related | [close](/crates/oxide-sketch/src/geom/boolean/close.md) |
| related | [area](/crates/oxide-sketch/src/geom/boolean/area.md) |
| related | [empty_inputs_return_empty](/crates/oxide-sketch/src/geom/boolean/empty_inputs_return_empty.md) |
| related | [full_overlap_returns_subject](/crates/oxide-sketch/src/geom/boolean/full_overlap_returns_subject.md) |
| related | [no_overlap_returns_empty](/crates/oxide-sketch/src/geom/boolean/no_overlap_returns_empty.md) |
| related | [partial_overlap_quarter_square](/crates/oxide-sketch/src/geom/boolean/partial_overlap_quarter_square.md) |
| related | [concave_subject_against_convex_clip](/crates/oxide-sketch/src/geom/boolean/concave_subject_against_convex_clip.md) |
| related | [cw_clip_winding_normalised](/crates/oxide-sketch/src/geom/boolean/cw_clip_winding_normalised.md) |
| related | [point_on_clip_edge_kept](/crates/oxide-sketch/src/geom/boolean/point_on_clip_edge_kept.md) |
