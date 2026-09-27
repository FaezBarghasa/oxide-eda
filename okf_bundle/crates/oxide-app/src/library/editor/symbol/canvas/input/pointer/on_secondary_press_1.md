---
okf_version: "0.2"
type: Function
title: on_secondary_press
description: "Right/Middle press: **Right** cancels an in-progress multi-click"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_press_1
language: rust
---

# on_secondary_press

Right/Middle press: **Right** cancels an in-progress multi-click

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn on_secondary_press(
        &self,
        state: &mut CanvasState,
        button: &mouse::Button,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<CanvasAction>>
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Right/Middle press: **Right** cancels an in-progress multi-click
draw (Place Polygon's stash included), else starts a pan.
**Middle** never cancels Place Polygon — it always proceeds
straight to arming a pan with the stash left intact. The
vertex stash has no undo, so treating a middle-button pan
attempt as a cancel would silently destroy every click the
user had placed so far.

## Source
Lines 28–79 in `crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.md) |
| calls | [should_cancel_polygon_placement](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/should_cancel_polygon_placement.md) |
