---
okf_version: "0.2"
type: Function
title: on_wheel_scrolled
description: "Handle a scroll-wheel tick: publish a cursor-anchored `Zoom`."
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/camera/on_wheel_scrolled_1
language: rust
---

# on_wheel_scrolled

Handle a scroll-wheel tick: publish a cursor-anchored `Zoom`.

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn on_wheel_scrolled(
        &self,
        delta: &mouse::ScrollDelta,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<CanvasAction>>
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Handle a scroll-wheel tick: publish a cursor-anchored `Zoom`.

## Source
Lines 12–34 in `crates/oxide-app/src/library/editor/symbol/canvas/input/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/library/editor/symbol/canvas/input/camera.md) |
