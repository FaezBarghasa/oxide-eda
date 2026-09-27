---
okf_version: "0.2"
type: Function
title: on_cursor_moved
description: "Cursor move: pan / handle-drag / item-drag / rubber-band +"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_moved_1
language: rust
---

# on_cursor_moved

Cursor move: pan / handle-drag / item-drag / rubber-band +

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn on_cursor_moved(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<CanvasAction>>
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Cursor move: pan / handle-drag / item-drag / rubber-band +
multi-click preview tracking / idle coordinate readout.

## Source
Lines 137–298 in `crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.md) |
| calls | [pan_moved_past_threshold](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/pan_moved_past_threshold.md) |
| calls | [world_for](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_for.md) |
| calls | [world_unsnapped](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_unsnapped.md) |
| calls | [unwrap_angle](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/unwrap_angle.md) |
