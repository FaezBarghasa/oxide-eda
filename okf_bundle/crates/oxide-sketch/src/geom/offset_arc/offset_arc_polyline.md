---
okf_version: "0.2"
type: Function
title: offset_arc_polyline
description: "Offset an arc-aware closed polyline by signed distance `d`."
resource: crates/oxide-sketch/src/geom/offset_arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/offset_arc/offset_arc_polyline
language: rust
---

# offset_arc_polyline

Offset an arc-aware closed polyline by signed distance `d`.

## Signature

```rust
pub fn offset_arc_polyline(
    elements: &[PolyElement],
    d: f64,
    polygon_ccw: bool,
) -> Vec<PolyElement>
```

## Visibility

- `pub`

## Docstring

Offset an arc-aware closed polyline by signed distance `d`.
Positive grows outward, negative shrinks inward. Returns a
new sequence of `PolyElement`s.

Approximation note: convex corner-bridge arcs are emitted
directly as `PolyElement::Arc` so the result stays arc-aware.
Concave corners bevel; the resulting local self-intersection
must be cleaned up by `polygon_op` if a topologically clean
offset is needed.

`polygon_ccw` tells us the original polyline's winding so the
outward normal direction is unambiguous. Caller computes this
via the shoelace area on the polyline endpoints.

## Source
Lines 125–171 in `crates/oxide-sketch/src/geom/offset_arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [offset_arc](/crates/oxide-sketch/src/geom/offset_arc.md) |
| calls | [offset_element](/crates/oxide-sketch/src/geom/offset_arc/offset_element.md) |
| called_by | [arc_grows_radius_on_outward_offset](/crates/oxide-sketch/src/geom/offset_arc/arc_grows_radius_on_outward_offset.md) |
| called_by | [arc_shrinks_on_inward_offset](/crates/oxide-sketch/src/geom/offset_arc/arc_shrinks_on_inward_offset.md) |
| called_by | [empty_input_empty_output](/crates/oxide-sketch/src/geom/offset_arc/empty_input_empty_output.md) |
| called_by | [line_only_polygon_offsets_each_edge](/crates/oxide-sketch/src/geom/offset_arc/line_only_polygon_offsets_each_edge.md) |
