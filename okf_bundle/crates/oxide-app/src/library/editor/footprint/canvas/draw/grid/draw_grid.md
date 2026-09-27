---
okf_version: "0.2"
type: Function
title: draw_grid
description: Render a grid of straight lines stroked at the given step. All
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/grid/draw_grid
language: rust
---

# draw_grid

Render a grid of straight lines stroked at the given step. All

## Signature

```rust
pub(super) fn draw_grid(
    frame: &mut canvas::Frame,
    bounds: Rectangle,
    offset: Point,
    step: f32,
    color: Color,
)
```

## Visibility

- `pub(super)`

## Docstring

Render a grid of straight lines stroked at the given step. All
lines compose into a single Path so iced tessellates once per
frame — the per-line `frame.stroke` loop was the dominant cost
when panning an empty footprint canvas.

## Source
Lines 10–33 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/library/editor/footprint/canvas/draw/grid.md) |
