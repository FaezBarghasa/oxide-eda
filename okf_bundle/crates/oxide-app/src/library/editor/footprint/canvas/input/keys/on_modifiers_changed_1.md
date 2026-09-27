---
okf_version: "0.2"
type: Function
title: on_modifiers_changed
description: "v0.27 — mirror `ModifiersChanged` into `cstate` so the mouse"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/keys/on_modifiers_changed_1
language: rust
---

# on_modifiers_changed

v0.27 — mirror `ModifiersChanged` into `cstate` so the mouse

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn on_modifiers_changed(
        &self,
        cstate: &mut FootprintCanvasState,
        mods: &keyboard::Modifiers,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.27 — mirror `ModifiersChanged` into `cstate` so the mouse
press handlers can branch on Ctrl/Cmd + Shift (iced 0.14 mouse
events don't carry modifiers). Returns None so the rest of the
app still receives the event.

## Source
Lines 21–28 in `crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys.md) |
