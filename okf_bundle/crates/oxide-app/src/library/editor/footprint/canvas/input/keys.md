---
okf_version: "0.2"
type: Module
title: keys
description: "Keyboard input — modifier tracking, clipboard shortcuts, Space /"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/keys
language: rust
---

# keys

Keyboard input — modifier tracking, clipboard shortcuts, Space /

## Docstring

Keyboard input — modifier tracking, clipboard shortcuts, Space /
X (rotate / flip) on the selected pad, and the Sketch-mode live
numeric placement input.

The catch-all char forwarding reads the first codepoint of the
event's platform `text`; the dispatcher extracts it so this module
never has to name iced's `SmolStr` type.

## Relationships

| Type | Target |
|------|--------|
| related | [on_modifiers_changed](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/on_modifiers_changed.md) |
| related | [on_key_pressed](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/on_key_pressed.md) |
| related | [key_clipboard](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_clipboard.md) |
| related | [key_rotate_flip](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_rotate_flip.md) |
| related | [key_sketch_input](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_sketch_input.md) |
| related | [on_modifiers_changed](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/on_modifiers_changed.md) |
| related | [on_key_pressed](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/on_key_pressed.md) |
| related | [key_clipboard](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_clipboard.md) |
| related | [key_rotate_flip](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_rotate_flip.md) |
| related | [key_sketch_input](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_sketch_input.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
