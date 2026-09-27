---
okf_version: "0.2"
type: Function
title: update_cursor_moved
description: "Cursor motion: pan, track drag-move / box-select, hover reporting."
resource: crates/oxide-app/src/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/pointer/update_cursor_moved
language: rust
---

# update_cursor_moved

Cursor motion: pan, track drag-move / box-select, hover reporting.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn update_cursor_moved(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Cursor motion: pan, track drag-move / box-select, hover reporting.

## Source
Lines 280–369 in `crates/oxide-app/src/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/canvas/input/pointer.md) |
| calls | [CanvasEvent](/crates/oxide-app/src/canvas/mod/CanvasEvent.md) |
