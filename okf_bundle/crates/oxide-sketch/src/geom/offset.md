---
okf_version: "0.2"
type: Module
title: offset
description: Polygon offset (Minkowski-style outward / inward expansion).
resource: crates/oxide-sketch/src/geom/offset.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/offset
language: rust
---

# offset

Polygon offset (Minkowski-style outward / inward expansion).

## Docstring

Polygon offset (Minkowski-style outward / inward expansion).

Given a closed polygon and a signed offset distance `d`:
- `d > 0` grows the polygon outward (used for soldermask
expansion, courtyard buffer).
- `d < 0` shrinks it inward (used for pad keepout zones).

Two corner styles are supported:
- [`CornerStyle::Round`] — replaces each convex corner with an
arc of radius `|d|`. The arc is sampled into segments at a
user-configurable `arc_segments` count so the output stays a
plain polygon.
- [`CornerStyle::Miter`] — extends the offset edges until they
meet, with a `miter_limit` that falls back to a bevel when
the join would explode (sharp interior angles).

Inward offsets that exceed the polygon's smallest local radius
produce self-intersections. The implementation does NOT clean
those up — callers needing a polygon-boolean cleanup pass must
use the Phase 2 boolean module (queued).

## Relationships

| Type | Target |
|------|--------|
| related | [CornerStyle](/crates/oxide-sketch/src/geom/offset/CornerStyle.md) |
| related | [default](/crates/oxide-sketch/src/geom/offset/default.md) |
| related | [default](/crates/oxide-sketch/src/geom/offset/default.md) |
| related | [offset_polygon](/crates/oxide-sketch/src/geom/offset/offset_polygon.md) |
| related | [p](/crates/oxide-sketch/src/geom/offset/p.md) |
| related | [close](/crates/oxide-sketch/src/geom/offset/close.md) |
| related | [empty_polygon_returns_empty](/crates/oxide-sketch/src/geom/offset/empty_polygon_returns_empty.md) |
| related | [square_outward_miter_grows_corners](/crates/oxide-sketch/src/geom/offset/square_outward_miter_grows_corners.md) |
| related | [square_round_offset_emits_arcs](/crates/oxide-sketch/src/geom/offset/square_round_offset_emits_arcs.md) |
| related | [cw_input_grows_outward_too](/crates/oxide-sketch/src/geom/offset/cw_input_grows_outward_too.md) |
| related | [negative_offset_shrinks](/crates/oxide-sketch/src/geom/offset/negative_offset_shrinks.md) |
