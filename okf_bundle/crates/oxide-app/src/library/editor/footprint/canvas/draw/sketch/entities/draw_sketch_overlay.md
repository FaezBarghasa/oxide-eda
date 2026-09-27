---
okf_version: "0.2"
type: Function
title: draw_sketch_overlay
description: Render the sketch entities (Phase 6.2). Points draw as small
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/draw_sketch_overlay
language: rust
---

# draw_sketch_overlay

Render the sketch entities (Phase 6.2). Points draw as small

## Signature

```rust
pub(in crate::library::editor::footprint::canvas::draw) fn draw_sketch_overlay(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    sketch: &oxide_sketch::SketchData,
    state: &FootprintEditorState,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas::draw)`

## Docstring

Render the sketch entities (Phase 6.2). Points draw as small
filled circles, Lines stroke between their endpoints (dashed if
`construction == true`), Circles stroke the radius circle, Arcs
stroke a polyline approximation between start/end. DOF colour
drives the tint.

## Source
Lines 19–323 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entities](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [draw_constraint_icons](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/draw_constraint_icons.md) |
| calls | [draw_filled_closed_loops](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/draw_filled_closed_loops.md) |
| calls | [point_world](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/point_world.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| called_by | [draw_sketch_overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_overlays.md) |
