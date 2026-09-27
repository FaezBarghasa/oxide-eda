---
okf_version: "0.2"
type: Function
title: get_node
description: Get a node by path (immutable).
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view/get_node
language: rust
---

# get_node

Get a node by path (immutable).

## Signature

```rust
pub fn get_node(roots: &'a [TreeNode], path: &[usize]) -> Option<&'a TreeNode>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Get a node by path (immutable).

## Source
Lines 629–638 in `crates/oxide-widgets/src/tree_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_view](/crates/oxide-widgets/src/tree_view.md) |
| called_by | [handle_dock_project_navigation_panel_message](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handle_dock_project_navigation_panel_message.md) |
| called_by | [tree_path_to_file_path](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/tree_path_to_file_path.md) |
| called_by | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
