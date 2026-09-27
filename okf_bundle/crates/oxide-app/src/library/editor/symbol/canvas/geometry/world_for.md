---
okf_version: "0.2"
type: Function
title: world_for
description: "Convert screen coords → world-mm via the camera, then snap to"
resource: crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_for
language: rust
---

# world_for

Convert screen coords → world-mm via the camera, then snap to

## Signature

```rust
pub(super) fn world_for(
    canvas: &SymbolCanvas<'_>,
    sx: f32,
    sy: f32,
    bounds: Rectangle,
) -> (f64, f64)
```

## Visibility

- `pub(super)`

## Docstring

Convert screen coords → world-mm via the camera, then snap to
the symbol-canvas grid. The canvas's Standard y-flip happens at
the world↔screen boundary inside `world_to_screen` /
`screen_to_world`; we mirror it here so screen-down → world-up.

## Source
Lines 73–91 in `crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/symbol/canvas/geometry.md) |
| called_by | [on_cursor_moved](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_moved.md) |
| called_by | [on_left_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press.md) |
