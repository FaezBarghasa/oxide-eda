---
okf_version: "0.2"
type: Function
title: update_wheel_scrolled
description: Mouse-wheel zoom about the cursor.
resource: crates/oxide-app/src/canvas/input/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/camera/update_wheel_scrolled
language: rust
---

# update_wheel_scrolled

Mouse-wheel zoom about the cursor.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn update_wheel_scrolled(
        &self,
        delta: &mouse::ScrollDelta,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Mouse-wheel zoom about the cursor.

## Source
Lines 23–46 in `crates/oxide-app/src/canvas/input/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/canvas/input/camera.md) |
| calls | [CanvasEvent](/crates/oxide-app/src/canvas/mod/CanvasEvent.md) |
