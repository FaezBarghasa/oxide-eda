---
okf_version: "0.2"
type: Function
title: on_button_released
description: "---- Button released --------------------------------------------"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/on_button_released_1
language: rust
---

# on_button_released

---- Button released --------------------------------------------

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn on_button_released(
        &self,
        cstate: &mut FootprintCanvasState,
        button: &mouse::Button,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

---- Button released --------------------------------------------

## Source
Lines 241–255 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
