---
okf_version: "0.2"
type: Class
title: ProjectTreeAction
description: Concrete actions dispatched when the user picks a menu item in the
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/ProjectTreeAction
language: rust
---

# ProjectTreeAction

Concrete actions dispatched when the user picks a menu item in the

## Signature

```rust
pub enum ProjectTreeAction
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Concrete actions dispatched when the user picks a menu item in the
Projects-panel tree-view context menu.
[derive(Debug, Clone)]

## Source
Lines 193–269 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
| called_by | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
| called_by | [add_new_entries](/crates/oxide-app/src/app/view/context_menu/submenu/add_new_entries.md) |
| called_by | [project_options_modal_opens_with_metadata_then_closes](/crates/oxide-app/tests/regression/project/project_options_modal_opens_with_metadata_then_closes.md) |
