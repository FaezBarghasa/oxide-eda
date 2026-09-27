---
okf_version: "0.2"
type: Module
title: helpers
description: "Mint primitives shared across `mint_*_pad_geometry` functions."
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers
language: rust
---

# helpers

Mint primitives shared across `mint_*_pad_geometry` functions.

## Docstring

Mint primitives shared across `mint_*_pad_geometry` functions.

Each `push_*` helper allocates a fresh `SketchEntityId`, builds the
matching `Entity`, and pushes it onto `sketch.entities`. The
`_construction` variants set `entity.construction = true` so the
bake skips them. Together these collapse ~30 near-identical
3-line blocks across the mint pipeline into single calls.

## Relationships

| Type | Target |
|------|--------|
| related | [push_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_point.md) |
| related | [push_construction_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_construction_point.md) |
| related | [push_line](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_line.md) |
| related | [push_construction_line](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_construction_line.md) |
| related | [push_arc_ccw](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_arc_ccw.md) |
| related | [bbox_corner_points](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/bbox_corner_points.md) |
| related | [set_point_xy](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/set_point_xy.md) |
| related | [bind_shape_param](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/bind_shape_param.md) |
