---
okf_version: "0.2"
type: Function
title: update_right_released
description: "Right-release: stop pan, context menu, or cancel drawing."
resource: crates/oxide-app/src/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/pointer/update_right_released_1
language: rust
---

# update_right_released

Right-release: stop pan, context menu, or cancel drawing.

## Signature

```rust
pub(in crate::canvas) fn update_right_released(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>>
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Right-release: stop pan, context menu, or cancel drawing.

## Source
Lines 240–267 in `crates/oxide-app/src/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/canvas/input/pointer.md) |
| calls | [Tool](/crates/oxide-app/src/app/documents/Tool.md) |
