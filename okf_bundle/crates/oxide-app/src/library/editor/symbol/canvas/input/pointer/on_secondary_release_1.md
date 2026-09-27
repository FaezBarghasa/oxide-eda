---
okf_version: "0.2"
type: Function
title: on_secondary_release
description: "Right/Middle release: end the pan. A **right**-release that did"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_release_1
language: rust
---

# on_secondary_release

Right/Middle release: end the pan. A **right**-release that did

## Signature

```rust
pub(in crate::library::editor::symbol::canvas) fn on_secondary_release(
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

Right/Middle release: end the pan. A **right**-release that did
not pan opens the context menu (pin → graphic → empty hit
priority), window-absolute coords, mirroring the footprint
canvas's `on_secondary_released`. Middle-release never opens the
menu. Note: when the matching press instead cancelled an
in-progress Place Polygon / multi-click draw (see
`on_secondary_press`), `state.panning` was never armed, so this
release naturally falls through to a no-op — placement-cancel
wins over opening the menu with no extra checks needed here.

## Source
Lines 90–133 in `crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.md) |
| calls | [world_unsnapped](/crates/oxide-app/src/library/editor/symbol/canvas/geometry/world_unsnapped.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
