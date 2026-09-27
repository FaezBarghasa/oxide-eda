---
okf_version: "0.2"
type: Function
title: view_context_menu
description: Build the right-click context menu card for the active symbol
resource: crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/mod/view_context_menu
language: rust
---

# view_context_menu

Build the right-click context menu card for the active symbol

## Signature

```rust
pub fn view_context_menu(
    editor: &'a SymbolEditorState,
    tokens: &'a ThemeTokens,
    path: &'a Path,
) -> Option<iced::Element<'a, LibraryMessage>>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the right-click context menu card for the active symbol
editor. Returns `None` when the menu is closed.

## Source
Lines 38–52 in `crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/context_menu/mod.md) |
| calls | [build_symbol_context_menu_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
