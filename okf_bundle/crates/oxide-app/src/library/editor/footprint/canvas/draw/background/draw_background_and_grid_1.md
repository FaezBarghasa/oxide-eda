---
okf_version: "0.2"
type: Function
title: draw_background_and_grid
description: Background fill + fine/coarse grid. Sketch mode flips to a
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/background.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/background/draw_background_and_grid_1
language: rust
---

# draw_background_and_grid

Background fill + fine/coarse grid. Sketch mode flips to a

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn draw_background_and_grid(
        &self,
        frame: &mut canvas::Frame,
        cstate: &FootprintCanvasState,
        bounds: Rectangle,
    )
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

Background fill + fine/coarse grid. Sketch mode flips to a
Fusion-style white canvas + single mid-grey grid; Pads mode
keeps the dark theme + 2-tier grid.

## Source
Lines 15–83 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/background.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [background](/crates/oxide-app/src/library/editor/footprint/canvas/draw/background.md) |
| calls | [draw_grid_dots](/crates/oxide-app/src/library/editor/footprint/canvas/draw/grid/draw_grid_dots.md) |
