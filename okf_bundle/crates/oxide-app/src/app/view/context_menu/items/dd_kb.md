---
okf_version: "0.2"
type: Function
title: dd_kb
description: "A keyboard-command row: clicking it dispatches `action` through the"
resource: crates/oxide-app/src/app/view/context_menu/items.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/items/dd_kb
language: rust
---

# dd_kb

A keyboard-command row: clicking it dispatches `action` through the

## Signature

```rust
pub(super) fn dd_kb(
    icon: Option<Handle>,
    label: &str,
    shortcut: &str,
    action: ContextAction,
) -> DropdownEntry<Message>
```

## Visibility

- `pub(super)`

## Docstring

A keyboard-command row: clicking it dispatches `action` through the
context-menu message bridge. `shortcut` renders right-aligned (empty
= no hint); `icon` is optional (the widget still reserves the glyph
column so labels stay aligned).

## Source
Lines 24–36 in `crates/oxide-app/src/app/view/context_menu/items.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [items](/crates/oxide-app/src/app/view/context_menu/items.md) |
| calls | [dd_msg](/crates/oxide-app/src/app/view/context_menu/items/dd_msg.md) |
| called_by | [canvas_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/canvas_menu_entries.md) |
| called_by | [align_entries](/crates/oxide-app/src/app/view/context_menu/submenu/align_entries.md) |
| called_by | [place_entries](/crates/oxide-app/src/app/view/context_menu/submenu/place_entries.md) |
