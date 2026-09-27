---
okf_version: "0.2"
type: Function
title: canvas_menu_entries
description: Pure canvas-menu data builder. Rows and their enable state are derived
resource: crates/oxide-app/src/app/view/context_menu/menus.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/menus/canvas_menu_entries
language: rust
---

# canvas_menu_entries

Pure canvas-menu data builder. Rows and their enable state are derived

## Signature

```rust
pub(super) fn canvas_menu_entries(
    tid: ThemeId,
    tokens: &ThemeTokens,
    has_selection: bool,
    child_sheet_selected: bool,
    active_submenu: Option<ContextSubmenu>,
    sc: &CanvasShortcuts,
) -> Vec<DropdownEntry<Message>>
```

## Visibility

- `pub(super)`

## Docstring

Pure canvas-menu data builder. Rows and their enable state are derived
only from the passed selection flags + shortcut hints, so the menu's
shape is unit-testable without a window.

## Source
Lines 183–363 in `crates/oxide-app/src/app/view/context_menu/menus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menus](/crates/oxide-app/src/app/view/context_menu/menus.md) |
| calls | [dd_disabled](/crates/oxide-app/src/app/view/context_menu/items/dd_disabled.md) |
| calls | [dd_msg](/crates/oxide-app/src/app/view/context_menu/items/dd_msg.md) |
| calls | [submenu_launcher](/crates/oxide-app/src/app/view/context_menu/items/submenu_launcher.md) |
| calls | [dd_kb](/crates/oxide-app/src/app/view/context_menu/items/dd_kb.md) |
| called_by | [view_context_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_context_menu.md) |
| called_by | [canvas_child_sheet_adds_open_row](/crates/oxide-app/src/app/view/context_menu/tests/canvas_child_sheet_adds_open_row.md) |
| called_by | [canvas_menu_grows_and_gates_on_selection](/crates/oxide-app/src/app/view/context_menu/tests/canvas_menu_grows_and_gates_on_selection.md) |
