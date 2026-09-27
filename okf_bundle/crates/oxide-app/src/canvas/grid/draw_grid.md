---
okf_version: "0.2"
type: Function
title: draw_grid
resource: crates/oxide-app/src/canvas/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/grid/draw_grid
language: rust
---

# draw_grid

## Signature

```rust
pub fn draw_grid(
    frame: &mut canvas::Frame,
    camera: &Camera,
    grid_mm: f32,
    bounds: iced::Rectangle,
    color: Color,
    page_w: f32,
    page_h: f32,
    style: GridStyle,
)
```

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "8 arguments: camera + grid metrics + page bounds + colour + glyph style, all per-frame draw inputs"
)`

## Visibility

- `pub`

## Source
Lines 84–280 in `crates/oxide-app/src/canvas/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/canvas/grid.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
