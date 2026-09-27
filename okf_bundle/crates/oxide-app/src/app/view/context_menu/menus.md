---
okf_version: "0.2"
type: Module
title: menus
description: "Right-click context-menu builders for the canvas and tab strip, plus"
resource: crates/oxide-app/src/app/view/context_menu/menus.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/menus
language: rust
---

# menus

Right-click context-menu builders for the canvas and tab strip, plus

## Docstring

Right-click context-menu builders for the canvas and tab strip, plus
the grid-picker popup. (The project-tree menu lives in the sibling
`project_tree` module.)

Data-to-view (#269): each menu is assembled as a pure
`Vec<DropdownEntry<Message>>` and handed to the shared
`oxide_widgets::active_bar_dropdown` widget for rendering, so the
canvas / project-tree / tab menus share ONE row renderer with the
schematic + footprint active bars (ADR-0003). The pure `*_entries`
builders carry the data (labels, actions, enable state) and are unit-
tested without a GPU; the thin `&self` `view_*` shims resolve app
state (selection, shortcuts) and call the widget.

## Relationships

| Type | Target |
|------|--------|
| related | [CanvasShortcuts](/crates/oxide-app/src/app/view/context_menu/menus/CanvasShortcuts.md) |
| related | [view_grid_picker_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_grid_picker_menu.md) |
| related | [view_context_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_context_menu.md) |
| related | [view_tab_context_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_tab_context_menu.md) |
| related | [view_grid_picker_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_grid_picker_menu.md) |
| related | [view_context_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_context_menu.md) |
| related | [view_tab_context_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_tab_context_menu.md) |
| related | [canvas_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/canvas_menu_entries.md) |
| related | [tab_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/tab_menu_entries.md) |
