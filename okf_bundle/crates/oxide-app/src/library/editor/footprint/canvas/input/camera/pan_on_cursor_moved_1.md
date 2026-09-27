---
okf_version: "0.2"
type: Function
title: pan_on_cursor_moved
description: "Middle/right-drag pan step. Returns `Some(action)` when a pan"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/camera/pan_on_cursor_moved_1
language: rust
---

# pan_on_cursor_moved

Middle/right-drag pan step. Returns `Some(action)` when a pan

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn pan_on_cursor_moved(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

Middle/right-drag pan step. Returns `Some(action)` when a pan
was applied (mirrors the original always-returns behaviour of
the pan branch), `None` when no pan is in flight so the caller
falls through to the rest of the cursor-move handling.

## Source
Lines 122–178 in `crates/oxide-app/src/library/editor/footprint/canvas/input/camera.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [camera](/crates/oxide-app/src/library/editor/footprint/canvas/input/camera.md) |
