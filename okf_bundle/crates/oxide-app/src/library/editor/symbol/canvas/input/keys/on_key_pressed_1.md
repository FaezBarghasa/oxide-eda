---
okf_version: "0.2"
type: Function
title: on_key_pressed
description: Handle a key press over the canvas.
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/keys/on_key_pressed_1
language: rust
---

# on_key_pressed

Handle a key press over the canvas.

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn on_key_pressed(
        &self,
        state: &mut CanvasState,
        key: &iced::keyboard::Key,
        modifiers: &iced::keyboard::Modifiers,
    ) -> Option<canvas::Action<CanvasAction>>
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Handle a key press over the canvas.

## Source
Lines 11–130 in `crates/oxide-app/src/library/editor/symbol/canvas/input/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-app/src/library/editor/symbol/canvas/input/keys.md) |
