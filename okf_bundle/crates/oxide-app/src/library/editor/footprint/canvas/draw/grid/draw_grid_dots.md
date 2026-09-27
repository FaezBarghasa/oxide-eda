---
okf_version: "0.2"
type: Function
title: draw_grid_dots
description: v0.18.22 — dotted grid variant. One filled square per intersection
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/grid/draw_grid_dots
language: rust
---

# draw_grid_dots

v0.18.22 — dotted grid variant. One filled square per intersection

## Signature

```rust
pub(super) fn draw_grid_dots(
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

v0.18.22 — dotted grid variant. One filled square per intersection
rendered as a single `frame.fill` over a composed path so the cost
matches `draw_grid`'s single-stroke design. The dot side is
1.4 px (looks like a 1×1 dot at typical DPI without disappearing
at fractional pixels).

## Source
Lines 40–65 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/library/editor/footprint/canvas/draw/grid.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| called_by | [draw_background_and_grid](/crates/oxide-app/src/library/editor/footprint/canvas/draw/background/draw_background_and_grid.md) |
