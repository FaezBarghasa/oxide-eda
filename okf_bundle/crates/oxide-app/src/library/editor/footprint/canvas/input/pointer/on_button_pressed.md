---
okf_version: "0.2"
type: Function
title: on_button_pressed
description: "---- Button pressed ---------------------------------------------"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_button_pressed
language: rust
---

# on_button_pressed

---- Button pressed ---------------------------------------------

## Signature

```rust
impl FootprintCanvas<'_> { pub(in crate::library::editor::footprint::canvas) fn on_button_pressed(
        &self,
        cstate: &mut FootprintCanvasState,
        button: &mouse::Button,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

---- Button pressed ---------------------------------------------

## Source
Lines 25–41 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
