---
okf_version: "0.2"
type: Function
title: draw_touching_line_ghost
description: v0.27 — Touching Line ghost — second-endpoint preview tracking
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_touching_line_ghost_1
language: rust
---

# draw_touching_line_ghost

v0.27 — Touching Line ghost — second-endpoint preview tracking

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_touching_line_ghost(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — Touching Line ghost — second-endpoint preview tracking
the cursor after the first click.

## Source
Lines 209–230 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
