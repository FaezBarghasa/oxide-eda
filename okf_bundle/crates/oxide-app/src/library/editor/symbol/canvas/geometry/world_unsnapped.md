---
okf_version: "0.2"
type: Function
title: world_unsnapped
description: "Same as `world_for` but without the snap — used by the cursor"
resource: crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_unsnapped
language: rust
---

# world_unsnapped

Same as `world_for` but without the snap — used by the cursor

## Signature

```rust
pub(super) fn world_unsnapped(
    canvas: &SymbolCanvas<'_>,
    sx: f32,
    sy: f32,
    bounds: Rectangle,
) -> (f64, f64)
```

## Visibility

- `pub(super)`

## Docstring

Same as `world_for` but without the snap — used by the cursor
readout so the status footer shows the unsnapped position the
user actually pointed at.

## Source
Lines 96–106 in `crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/symbol/canvas/geometry.md) |
| called_by | [draw_grid](/crates/oxide-app/src/library/editor/symbol/canvas/draw/background/draw_grid.md) |
| called_by | [on_cursor_moved](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_moved.md) |
| called_by | [on_secondary_release](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_release.md) |
| called_by | [on_left_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press.md) |
| called_by | [mouse_interaction](/crates/oxide-app/src/library/editor/symbol/canvas/mod/mouse_interaction.md) |
