---
okf_version: "0.2"
type: Function
title: on_cursor_moved
description: "---- Cursor moved -----------------------------------------------"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_cursor_moved_1
language: rust
---

# on_cursor_moved

---- Cursor moved -----------------------------------------------

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn on_cursor_moved(
        &self,
        cstate: &mut FootprintCanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

---- Cursor moved -----------------------------------------------

## Source
Lines 353–371 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
