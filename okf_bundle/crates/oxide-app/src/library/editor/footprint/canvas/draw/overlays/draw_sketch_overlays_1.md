---
okf_version: "0.2"
type: Function
title: draw_sketch_overlays
description: "v0.13.1 — sketch-entity overlay (entities, DOF arrows, tool"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_overlays_1
language: rust
---

# draw_sketch_overlays

v0.13.1 — sketch-entity overlay (entities, DOF arrows, tool

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_sketch_overlays(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.13.1 — sketch-entity overlay (entities, DOF arrows, tool
preview, snap glyph). Only in Sketch mode with a sketch present.

## Source
Lines 302–319 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.md) |
| calls | [draw_sketch_overlay](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/draw_sketch_overlay.md) |
| calls | [draw_dof_direction_arrows](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/arrows/draw_dof_direction_arrows.md) |
| calls | [draw_sketch_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_sketch_tool_preview.md) |
| calls | [draw_sketch_snap_glyph](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/snap/draw_sketch_snap_glyph.md) |
