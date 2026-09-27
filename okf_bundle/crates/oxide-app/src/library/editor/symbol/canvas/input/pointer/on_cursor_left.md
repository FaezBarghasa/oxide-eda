---
okf_version: "0.2"
type: Function
title: on_cursor_left
description: "Cursor left the canvas: end pan, clear the coordinate readout."
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_left
language: rust
---

# on_cursor_left

Cursor left the canvas: end pan, clear the coordinate readout.

## Signature

```rust
impl SymbolCanvas<'_> { pub(in crate::library::editor::symbol::canvas) fn on_cursor_left(
        &self,
        state: &mut CanvasState,
    ) -> Option<canvas::Action<CanvasAction>> }
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Cursor left the canvas: end pan, clear the coordinate readout.

## Source
Lines 301–312 in `crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.md) |
