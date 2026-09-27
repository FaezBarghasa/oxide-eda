---
okf_version: "0.2"
type: Module
title: submenu
description: Secondary context submenu body (Place / Align / Add New to Project)
resource: crates/oxide-app/src/app/view/context_menu/submenu.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/submenu
language: rust
---

# submenu

Secondary context submenu body (Place / Align / Add New to Project)

## Docstring

Secondary context submenu body (Place / Align / Add New to Project)
shown to the right of the canvas and project-tree context menus.

Data-to-view (#269): each submenu is a pure `Vec<DropdownEntry<Message>>`
rendered by the shared `oxide_widgets::active_bar_dropdown` widget, so
the flyout shares its row chrome with the parent menus and the active
bars (ADR-0003). Every Place / Align row dispatches an Active Bar action
via `ContextAction::ActiveBar(...)` so placement / transform pipelines
stay shared with the toolbar.

## Relationships

| Type | Target |
|------|--------|
| related | [place_entries](/crates/oxide-app/src/app/view/context_menu/submenu/place_entries.md) |
| related | [align_gate](/crates/oxide-app/src/app/view/context_menu/submenu/align_gate.md) |
| related | [align_entries](/crates/oxide-app/src/app/view/context_menu/submenu/align_entries.md) |
| related | [add_new_entries](/crates/oxide-app/src/app/view/context_menu/submenu/add_new_entries.md) |
| related | [view_context_submenu](/crates/oxide-app/src/app/view/context_menu/submenu/view_context_submenu.md) |
| related | [view_context_submenu](/crates/oxide-app/src/app/view/context_menu/submenu/view_context_submenu.md) |
