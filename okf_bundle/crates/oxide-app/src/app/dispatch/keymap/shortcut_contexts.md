---
okf_version: "0.2"
type: Function
title: shortcut_contexts
description: "Which shortcut contexts are live for a stroke typed in `window`"
resource: crates/oxide-app/src/app/dispatch/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/keymap/shortcut_contexts
language: rust
---

# shortcut_contexts

Which shortcut contexts are live for a stroke typed in `window`

## Signature

```rust
impl Oxide { fn shortcut_contexts(&self, window: Option<iced::window::Id>) -> Vec<ShortcutContext> }
```

## Docstring

Which shortcut contexts are live for a stroke typed in `window`
(#559).

These used to be derived from `document_state.active_tab` — the
MAIN window's tab — no matter where the stroke came from, so a
shortcut pressed in an undocked schematic window resolved against
the main window's footprint / symbol / library contexts.

## Source
Lines 106–158 in `crates/oxide-app/src/app/dispatch/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/app/dispatch/keymap.md) |
