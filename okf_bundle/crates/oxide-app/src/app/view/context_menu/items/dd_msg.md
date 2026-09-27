---
okf_version: "0.2"
type: Function
title: dd_msg
description: "A row that publishes an arbitrary `Message` on click."
resource: crates/oxide-app/src/app/view/context_menu/items.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/items/dd_msg
language: rust
---

# dd_msg

A row that publishes an arbitrary `Message` on click.

## Signature

```rust
pub(super) fn dd_msg(
    icon: Option<Handle>,
    label: &str,
    shortcut: &str,
    message: Message,
) -> DropdownEntry<Message>
```

## Visibility

- `pub(super)`

## Docstring

A row that publishes an arbitrary `Message` on click.

## Source
Lines 39–53 in `crates/oxide-app/src/app/view/context_menu/items.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [items](/crates/oxide-app/src/app/view/context_menu/items.md) |
| called_by | [dd_kb](/crates/oxide-app/src/app/view/context_menu/items/dd_kb.md) |
| called_by | [save_entry](/crates/oxide-app/src/app/view/context_menu/items/save_entry.md) |
| called_by | [canvas_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/canvas_menu_entries.md) |
| called_by | [tab_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/tab_menu_entries.md) |
| called_by | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
| called_by | [add_new_entries](/crates/oxide-app/src/app/view/context_menu/submenu/add_new_entries.md) |
| called_by | [dd_msg_row_carries_message_and_optional_shortcut](/crates/oxide-app/src/app/view/context_menu/tests/dd_msg_row_carries_message_and_optional_shortcut.md) |
