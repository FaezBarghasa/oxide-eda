---
okf_version: "0.2"
type: Function
title: offset_polygon
description: "Offset the closed polygon by signed distance `d`. Returns a new"
resource: crates/oxide-sketch/src/geom/offset.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/offset/offset_polygon
language: rust
---

# offset_polygon

Offset the closed polygon by signed distance `d`. Returns a new

## Signature

```rust
pub fn offset_polygon(polygon: &[Point2], d: f64, style: CornerStyle) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Offset the closed polygon by signed distance `d`. Returns a new
polygon walking the same winding direction as the input.

The algorithm:
1. For each edge `(p[i], p[i+1])`, compute the unit-perpendicular
`n_i` (CCW polygon → outward normal points right of edge
direction; the sign is normalised internally so positive
`d` always grows outward regardless of input winding).
2. Each edge's offset is `(p[i] + d * n_i, p[i+1] + d * n_i)`.
3. Adjacent offset edges are joined per `style`:
- `Round` — emit the corner vertex `p[i+1]`'s offset point
for the incoming edge, then sample an arc on the convex
side, then emit the offset point for the outgoing edge.
- `Miter` — intersect the two offset lines; clamp the join
distance against `miter_limit`.
4. Concave corners always bevel (intersecting offset lines
land inside the polygon, which the Round / Miter logic
degenerates into a clean two-edge join).

Degenerate inputs (< 3 vertices, zero-area, or self-intersecting)
return an empty Vec.

## Source
Lines 65–192 in `crates/oxide-sketch/src/geom/offset.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [offset](/crates/oxide-sketch/src/geom/offset.md) |
| calls | [signed_area](/crates/oxide-sketch/src/geom/predicates/signed_area.md) |
| called_by | [recompute_courtyard_outline](/crates/oxide-app/src/library/editor/footprint/state/mod/recompute_courtyard_outline.md) |
| called_by | [cw_input_grows_outward_too](/crates/oxide-sketch/src/geom/offset/cw_input_grows_outward_too.md) |
| called_by | [negative_offset_shrinks](/crates/oxide-sketch/src/geom/offset/negative_offset_shrinks.md) |
| called_by | [square_outward_miter_grows_corners](/crates/oxide-sketch/src/geom/offset/square_outward_miter_grows_corners.md) |
| called_by | [square_round_offset_emits_arcs](/crates/oxide-sketch/src/geom/offset/square_round_offset_emits_arcs.md) |
