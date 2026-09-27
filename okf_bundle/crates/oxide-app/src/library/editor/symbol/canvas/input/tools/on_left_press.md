---
okf_version: "0.2"
type: Function
title: on_left_press
description: "Handle a left mouse-button press: Select-tool hit-testing or a"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press
language: rust
---

# on_left_press

Handle a left mouse-button press: Select-tool hit-testing or a

## Signature

```rust
impl SymbolCanvas<'_> { pub(in crate::library::editor::symbol::canvas) fn on_left_press(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<CanvasAction>> }
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Handle a left mouse-button press: Select-tool hit-testing or a
placement-tool gesture, depending on `self.tool`.

## Source
Lines 16–228 in `crates/oxide-app/src/library/editor/symbol/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools.md) |
| calls | [world_for](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_for.md) |
| calls | [world_unsnapped](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_unsnapped.md) |
| calls | [hit_test_graphic_handle](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_handle.md) |
| calls | [item_in_selection](/crates/oxide-app/src/library/editor/symbol/canvas/mod/item_in_selection.md) |
| calls | [selection_anchor](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/selection_anchor.md) |
| calls | [arc_sweep_exceeds_full_turn](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/arc_sweep_exceeds_full_turn.md) |
