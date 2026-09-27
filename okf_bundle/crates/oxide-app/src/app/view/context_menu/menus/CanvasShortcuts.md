---
okf_version: "0.2"
type: Class
title: CanvasShortcuts
description: "Keyboard-shortcut hints for the canvas menu, resolved from the active"
resource: crates/oxide-app/src/app/view/context_menu/menus.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/menus/CanvasShortcuts
language: rust
---

# CanvasShortcuts

Keyboard-shortcut hints for the canvas menu, resolved from the active

## Signature

```rust
pub(super) struct CanvasShortcuts
```

## Visibility

- `pub(super)`

## Docstring

Keyboard-shortcut hints for the canvas menu, resolved from the active
keymap profile (with the historic Altium defaults as fallbacks) before
the pure builder runs.

## Methods

- `find`
- `cut`
- `copy`
- `paste`
- `smart_paste`

## Source
Lines 24–30 in `crates/oxide-app/src/app/view/context_menu/menus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menus](/crates/oxide-app/src/app/view/context_menu/menus.md) |
