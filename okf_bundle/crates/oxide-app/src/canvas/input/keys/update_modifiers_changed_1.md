---
okf_version: "0.2"
type: Function
title: update_modifiers_changed
description: Track Ctrl/Shift modifier state for multi-select.
resource: crates/oxide-app/src/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/keys/update_modifiers_changed_1
language: rust
---

# update_modifiers_changed

Track Ctrl/Shift modifier state for multi-select.

## Signature

```rust
pub(in crate::canvas) fn update_modifiers_changed(
        &self,
        state: &mut CanvasState,
        mods: &iced::keyboard::Modifiers,
    ) -> Option<canvas::Action<Message>>
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Track Ctrl/Shift modifier state for multi-select.

## Source
Lines 5–13 in `crates/oxide-app/src/canvas/input/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-app/src/canvas/input/keys.md) |
