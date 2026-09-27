---
okf_version: "0.2"
type: Function
title: tree_node_role
description: "Pure role detection for a project-tree node (see [`TreeNodeRole`])."
resource: crates/oxide-app/src/app/view/context_menu/project_tree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/project_tree/tree_node_role
language: rust
---

# tree_node_role

Pure role detection for a project-tree node (see [`TreeNodeRole`]).

## Signature

```rust
pub(super) fn tree_node_role(icon: &TreeIcon, path_len: usize, has_children: bool) -> TreeNodeRole
```

## Visibility

- `pub(super)`

## Docstring

Pure role detection for a project-tree node (see [`TreeNodeRole`]).

## Source
Lines 28–55 in `crates/oxide-app/src/app/view/context_menu/project_tree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_tree](/crates/oxide-app/src/app/view/context_menu/project_tree.md) |
| called_by | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
