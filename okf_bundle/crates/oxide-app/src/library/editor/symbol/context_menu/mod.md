---
okf_version: "0.2"
type: Module
title: context_menu
description: Right-click canvas context menu (VIEW) for the symbol editor.
resource: crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/mod
language: rust
---

# context_menu

Right-click canvas context menu (VIEW) for the symbol editor.

## Docstring

Right-click canvas context menu (VIEW) for the symbol editor.

Mirrors `library::editor::footprint::context_menu`'s mounting
contract 1:1 (window-absolute coords, dismiss-layer overlay — see
that module's doc comment for the rationale) but renders through
the generic `oxide_widgets::active_bar_dropdown` row renderer
(already shared by the app-level context menus and every editor's
active-bar dropdown — see `app/view/context_menu/items.rs`)
instead of a hand-built widget tree. `rows` is the declarative row
data this module flattens into `DropdownEntry`s; [`flatten`] is
the one place that conversion happens.

## Relationships

| Type | Target |
|------|--------|
| related | [view_context_menu](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/view_context_menu.md) |
| related | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| related | [row_submenu_matches](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/row_submenu_matches.md) |
| related | [submenu_msg_for_id](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/submenu_msg_for_id.md) |
| related | [wrap](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/wrap.md) |
| related | [submenu_header_entry](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/submenu_header_entry.md) |
| related | [leaf_entry](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/leaf_entry.md) |
