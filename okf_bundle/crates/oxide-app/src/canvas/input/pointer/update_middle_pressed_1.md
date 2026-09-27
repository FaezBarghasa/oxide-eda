---
okf_version: "0.2"
type: Function
title: update_middle_pressed
description: "Middle-press: start pan."
resource: crates/oxide-app/src/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/pointer/update_middle_pressed_1
language: rust
---

# update_middle_pressed

Middle-press: start pan.

## Signature

```rust
pub(in crate::canvas) fn update_middle_pressed(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>>
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Middle-press: start pan.

## Source
Lines 225–237 in `crates/oxide-app/src/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/canvas/input/pointer.md) |
