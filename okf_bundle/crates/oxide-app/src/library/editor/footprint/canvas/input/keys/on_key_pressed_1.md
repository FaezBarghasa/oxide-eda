---
okf_version: "0.2"
type: Function
title: on_key_pressed
description: "v0.24 Track D — key handling: clipboard shortcuts, rotate / flip,"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/keys/on_key_pressed_1
language: rust
---

# on_key_pressed

v0.24 Track D — key handling: clipboard shortcuts, rotate / flip,

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn on_key_pressed(
        &self,
        key: &keyboard::Key,
        modifiers: &keyboard::Modifiers,
        typed_char: Option<char>,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.24 Track D — key handling: clipboard shortcuts, rotate / flip,
then the Sketch-mode numeric placement input. `typed_char` is
the first codepoint of the event's platform text (or None).

## Source
Lines 33–46 in `crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys.md) |
