---
okf_version: "0.2"
type: Module
title: project_tree
description: Projects-panel tree-view right-click menu.
resource: crates/oxide-app/src/app/view/context_menu/project_tree.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/project_tree
language: rust
---

# project_tree

Projects-panel tree-view right-click menu.

## Docstring

Projects-panel tree-view right-click menu.

Data-to-view (#269): the menu is assembled as a `Vec<DropdownEntry>`
rendered by the shared `oxide_widgets::active_bar_dropdown` widget. The
clicked node's [`TreeNodeRole`] (pure, unit-tested) selects which item
set the `&self` builder produces.

## Relationships

| Type | Target |
|------|--------|
| related | [TreeNodeRole](/crates/oxide-app/src/app/view/context_menu/project_tree/TreeNodeRole.md) |
| related | [tree_node_role](/crates/oxide-app/src/app/view/context_menu/project_tree/tree_node_role.md) |
| related | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
| related | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
