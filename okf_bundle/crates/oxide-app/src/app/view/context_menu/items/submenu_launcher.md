---
okf_version: "0.2"
type: Function
title: submenu_launcher
description: "Submenu launcher row (`Place ›`, `Align ›`, `Add New to Project ›`)."
resource: crates/oxide-app/src/app/view/context_menu/items.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/items/submenu_launcher
language: rust
---

# submenu_launcher

Submenu launcher row (`Place ›`, `Align ›`, `Add New to Project ›`).

## Signature

```rust
pub(super) fn submenu_launcher(
    tokens: &ThemeTokens,
    icon: Option<Handle>,
    label: &str,
    kind: ContextSubmenu,
    active: bool,
) -> DropdownEntry<Message>
```

## Visibility

- `pub(super)`

## Docstring

Submenu launcher row (`Place ›`, `Align ›`, `Add New to Project ›`).

This is the one context-menu row the shared widget can't express as a
plain `Item`: it needs `mouse_area` on_enter/on_exit so the 200 ms
hover timer (`ContextMenuMsg::SubmenuTickHover`) can open the flyout
without a click, plus an active-state highlight so the user can see
which submenu is open. It therefore rides the widget's `Custom`
escape hatch as an owned `Element<'static>`. The row is styled to
match the widget's `Item` rows (20 px icon column, 13 pt label,
`[5, 12]` padding, hover background) so it sits flush with its
neighbours; the `›` chevron renders right-aligned at the unified
`SUBMENU_ARROW_SIZE`.

## Source
Lines 103–175 in `crates/oxide-app/src/app/view/context_menu/items.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [items](/crates/oxide-app/src/app/view/context_menu/items.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [canvas_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/canvas_menu_entries.md) |
| called_by | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
