---
okf_version: "0.2"
type: Function
title: view_context_menu
description: Build the right-click context menu card for the active footprint
resource: crates/oxide-app/src/library/editor/footprint/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/context_menu/view_context_menu
language: rust
---

# view_context_menu

Build the right-click context menu card for the active footprint

## Signature

```rust
pub fn view_context_menu(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
    path: &'a std::path::Path,
    has_clipboard: bool,
    keymap: &'a CompiledKeymap,
) -> Option<Element<'a, LibraryMessage>>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the right-click context menu card for the active footprint
editor. Returns `None` when the menu is closed.

## Source
Lines 54–341 in `crates/oxide-app/src/library/editor/footprint/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/context_menu.md) |
| calls | [shortcut_label](/crates/oxide-app/src/library/editor/footprint/context_menu/shortcut_label.md) |
| calls | [item_disabled](/crates/oxide-app/src/library/editor/footprint/context_menu/item_disabled.md) |
| calls | [separator](/crates/oxide-app/src/library/editor/footprint/context_menu/separator.md) |
| calls | [item_submenu_header](/crates/oxide-app/src/library/editor/footprint/context_menu/item_submenu_header.md) |
| calls | [item_indented](/crates/oxide-app/src/library/editor/footprint/context_menu/item_indented.md) |
| calls | [item_msg](/crates/oxide-app/src/library/editor/footprint/context_menu/item_msg.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
