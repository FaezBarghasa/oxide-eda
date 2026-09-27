---
okf_version: "0.2"
type: Function
title: key_clipboard
description: v0.26-F — Ctrl+X / Ctrl+C / Ctrl+V clipboard shortcuts.
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_clipboard_1
language: rust
---

# key_clipboard

v0.26-F — Ctrl+X / Ctrl+C / Ctrl+V clipboard shortcuts.

## Signature

```rust
fn key_clipboard(
        &self,
        key: &keyboard::Key,
        modifiers: &keyboard::Modifiers,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

v0.26-F — Ctrl+X / Ctrl+C / Ctrl+V clipboard shortcuts.
Mode-agnostic (works in Normal AND Sketch). Captures the event
so iced's global key subscription doesn't fire a duplicate.

## Source
Lines 51–80 in `crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys.md) |
