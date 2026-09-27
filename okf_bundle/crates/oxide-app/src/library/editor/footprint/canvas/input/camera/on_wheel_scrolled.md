---
okf_version: "0.2"
type: Function
title: on_wheel_scrolled
description: "Scroll-wheel zoom, anchored on the cursor."
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/camera/on_wheel_scrolled
language: rust
---

# on_wheel_scrolled

Scroll-wheel zoom, anchored on the cursor.

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn on_wheel_scrolled(
        &self,
        cstate: &mut FootprintCanvasState,
        delta: &mouse::ScrollDelta,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

Scroll-wheel zoom, anchored on the cursor.

## Source
Lines 72–116 in `crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera.md) |
