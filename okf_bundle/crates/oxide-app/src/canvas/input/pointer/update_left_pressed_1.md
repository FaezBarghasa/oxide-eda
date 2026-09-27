---
okf_version: "0.2"
type: Function
title: update_left_pressed
description: "Left-press: select, tool action, start box-select, or start drag-move."
resource: crates/oxide-app/src/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/pointer/update_left_pressed_1
language: rust
---

# update_left_pressed

Left-press: select, tool action, start box-select, or start drag-move.

## Signature

```rust
pub(in crate::canvas) fn update_left_pressed(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>>
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Left-press: select, tool action, start box-select, or start drag-move.

## Source
Lines 5–123 in `crates/oxide-app/src/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/canvas/input/pointer.md) |
| calls | [CanvasEvent](/crates/oxide-app/src/canvas/mod/CanvasEvent.md) |
