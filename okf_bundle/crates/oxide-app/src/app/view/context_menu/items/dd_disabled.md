---
okf_version: "0.2"
type: Function
title: dd_disabled
description: "A greyed, non-clickable row — used for \"coming soon\" stubs and gated"
resource: crates/oxide-app/src/app/view/context_menu/items.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/items/dd_disabled
language: rust
---

# dd_disabled

A greyed, non-clickable row — used for "coming soon" stubs and gated

## Signature

```rust
pub(super) fn dd_disabled(
    icon: Option<Handle>,
    label: &str,
    right: Option<&str>,
) -> DropdownEntry<Message>
```

## Visibility

- `pub(super)`

## Docstring

A greyed, non-clickable row — used for "coming soon" stubs and gated
actions. `right` is the optional right-column text (a keyboard hint,
a `vX.Y` version badge, or the `›` submenu chevron for placeholder
launchers). Built as a passive `DropdownItem` (no `on_press`).

## Source
Lines 59–72 in `crates/oxide-app/src/app/view/context_menu/items.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [items](/crates/oxide-app/src/app/view/context_menu/items.md) |
| called_by | [save_entry](/crates/oxide-app/src/app/view/context_menu/items/save_entry.md) |
| called_by | [canvas_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/canvas_menu_entries.md) |
| called_by | [tab_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/tab_menu_entries.md) |
| called_by | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
| called_by | [add_new_entries](/crates/oxide-app/src/app/view/context_menu/submenu/add_new_entries.md) |
| called_by | [align_entries](/crates/oxide-app/src/app/view/context_menu/submenu/align_entries.md) |
| called_by | [dd_disabled_row_is_passive](/crates/oxide-app/src/app/view/context_menu/tests/dd_disabled_row_is_passive.md) |
