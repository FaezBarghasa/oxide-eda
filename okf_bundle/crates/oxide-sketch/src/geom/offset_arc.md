---
okf_version: "0.2"
type: Module
title: offset_arc
description: Arc-aware polygon offset. Accepts a closed polyline whose
resource: crates/oxide-sketch/src/geom/offset_arc.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/offset_arc
language: rust
---

# offset_arc

Arc-aware polygon offset. Accepts a closed polyline whose

## Docstring

Arc-aware polygon offset. Accepts a closed polyline whose
segments are either straight Lines or circular Arcs, and
returns the offset polyline whose arc segments stay arcs (no
sampling-to-polyline-then-offset roundtrip that would lose
precision and emit excess vertices).

Each segment offsets independently:
- Straight segment `(a, b)` shifts by `d` along its outward
normal.
- Arc `(centre, radius, start_angle, end_angle, ccw)` becomes
a concentric arc with `radius + d` (or `radius − d` for an
inward offset on a CCW arc); same centre, same angular
range. For a CW arc the sign flips.

Convex breaks between consecutive offset segments are bridged
with a round-corner arc of radius `|d|`, matching the standard
Minkowski offset semantics. Concave breaks bevel — the local
self-intersection that follows is left for the boolean cleanup
pass to resolve.

## Relationships

| Type | Target |
|------|--------|
| related | [PolyElement](/crates/oxide-sketch/src/geom/offset_arc/PolyElement.md) |
| related | [end](/crates/oxide-sketch/src/geom/offset_arc/end.md) |
| related | [start_outward_normal](/crates/oxide-sketch/src/geom/offset_arc/start_outward_normal.md) |
| related | [end_outward_normal](/crates/oxide-sketch/src/geom/offset_arc/end_outward_normal.md) |
| related | [end](/crates/oxide-sketch/src/geom/offset_arc/end.md) |
| related | [start_outward_normal](/crates/oxide-sketch/src/geom/offset_arc/start_outward_normal.md) |
| related | [end_outward_normal](/crates/oxide-sketch/src/geom/offset_arc/end_outward_normal.md) |
| related | [unit_perp_outward](/crates/oxide-sketch/src/geom/offset_arc/unit_perp_outward.md) |
| related | [offset_arc_polyline](/crates/oxide-sketch/src/geom/offset_arc/offset_arc_polyline.md) |
| related | [offset_element](/crates/oxide-sketch/src/geom/offset_arc/offset_element.md) |
| related | [p](/crates/oxide-sketch/src/geom/offset_arc/p.md) |
| related | [close](/crates/oxide-sketch/src/geom/offset_arc/close.md) |
| related | [empty_input_empty_output](/crates/oxide-sketch/src/geom/offset_arc/empty_input_empty_output.md) |
| related | [line_only_polygon_offsets_each_edge](/crates/oxide-sketch/src/geom/offset_arc/line_only_polygon_offsets_each_edge.md) |
| related | [arc_grows_radius_on_outward_offset](/crates/oxide-sketch/src/geom/offset_arc/arc_grows_radius_on_outward_offset.md) |
| related | [arc_shrinks_on_inward_offset](/crates/oxide-sketch/src/geom/offset_arc/arc_shrinks_on_inward_offset.md) |
