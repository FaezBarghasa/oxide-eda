---
okf_version: "0.2"
type: Module
title: geometry
description: Free math / coordinate helpers for the symbol canvas — text and
resource: crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/geometry
language: rust
---

# geometry

Free math / coordinate helpers for the symbol canvas — text and

## Docstring

Free math / coordinate helpers for the symbol canvas — text and
stroke sizing at zoom, angle unwrapping, screen↔world conversion
with the canvas snap grid, colour packing, circle tessellation,
and selection-anchor lookup. Pure code motion out of `mod.rs`;
`pub(super)` so both the parent `canvas` module and its `input` /
`draw` submodules can reach them.

## Relationships

| Type | Target |
|------|--------|
| related | [text_size_px_from_mm](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/text_size_px_from_mm.md) |
| related | [stroke_px_at_zoom](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/stroke_px_at_zoom.md) |
| related | [unwrap_angle](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/unwrap_angle.md) |
| related | [stroke_world_mm](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/stroke_world_mm.md) |
| related | [screen_px_to_world_mm](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/screen_px_to_world_mm.md) |
| related | [to_rgba](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/to_rgba.md) |
| related | [circle_vertices](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/circle_vertices.md) |
| related | [world_for](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_for.md) |
| related | [world_unsnapped](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_unsnapped.md) |
| related | [selection_anchor](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/selection_anchor.md) |
