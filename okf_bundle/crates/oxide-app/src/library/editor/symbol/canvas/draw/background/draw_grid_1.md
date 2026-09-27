---
okf_version: "0.2"
type: Function
title: draw_grid
description: "Adaptive minor/major grid, honouring the View ▸ Grid style."
resource: crates/oxide-app/src/library/editor/symbol/canvas/draw/background.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/draw/background/draw_grid_1
language: rust
---

# draw_grid

Adaptive minor/major grid, honouring the View ▸ Grid style.

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn draw_grid(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
    )
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Adaptive minor/major grid, honouring the View ▸ Grid style.

## Source
Lines 21–208 in `crates/oxide-app/src/library/editor/symbol/canvas/draw/background.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [background](/crates/oxide-app/src/library/editor/symbol/canvas/draw/background.md) |
| calls | [world_unsnapped](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_unsnapped.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
