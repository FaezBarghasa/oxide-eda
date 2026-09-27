---
okf_version: "0.2"
type: Module
title: items
description: "Shared context-menu primitives — the pure `DropdownEntry` builders"
resource: crates/oxide-app/src/app/view/context_menu/items.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/items
language: rust
---

# items

Shared context-menu primitives — the pure `DropdownEntry` builders

## Docstring

Shared context-menu primitives — the pure `DropdownEntry` builders
(`dd_kb`, `dd_msg`, `dd_disabled`, `save_entry`), the hover-driven
submenu launcher (`submenu_launcher`, an owned `Custom` row), and the
two `&self` lookups the menu builders need (`keymap_shortcut_label`,
`library_node_path_from_tree`).

Data-to-view (#269): every context menu is a `Vec<DropdownEntry>` that
the shared `oxide_widgets::active_bar_dropdown` widget renders, so the
canvas / project-tree / tab menus share ONE row renderer with the
schematic + footprint active bars (ADR-0003). The widget owns all row
chrome (icon column, label, shortcut, hover, disabled greying), so the
hand-built row buttons that used to live here are gone.

## Relationships

| Type | Target |
|------|--------|
| related | [dd_kb](/crates/oxide-app/src/app/view/context_menu/items/dd_kb.md) |
| related | [dd_msg](/crates/oxide-app/src/app/view/context_menu/items/dd_msg.md) |
| related | [dd_disabled](/crates/oxide-app/src/app/view/context_menu/items/dd_disabled.md) |
| related | [save_entry](/crates/oxide-app/src/app/view/context_menu/items/save_entry.md) |
| related | [submenu_launcher](/crates/oxide-app/src/app/view/context_menu/items/submenu_launcher.md) |
| related | [library_node_path_from_tree](/crates/oxide-app/src/app/view/context_menu/items/library_node_path_from_tree.md) |
| related | [keymap_shortcut_label](/crates/oxide-app/src/app/view/context_menu/items/keymap_shortcut_label.md) |
| related | [library_node_path_from_tree](/crates/oxide-app/src/app/view/context_menu/items/library_node_path_from_tree.md) |
| related | [keymap_shortcut_label](/crates/oxide-app/src/app/view/context_menu/items/keymap_shortcut_label.md) |
