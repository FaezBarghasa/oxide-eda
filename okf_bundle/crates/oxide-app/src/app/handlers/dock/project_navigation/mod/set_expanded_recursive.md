---
okf_version: "0.2"
type: Function
title: set_expanded_recursive
description: "Recursively set every node's `expanded` state — used by"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/mod/set_expanded_recursive
language: rust
---

# set_expanded_recursive

Recursively set every node's `expanded` state — used by

## Signature

```rust
fn set_expanded_recursive(nodes: &mut [oxide_widgets::tree_view::TreeNode], expanded: bool)
```

## Docstring

Recursively set every node's `expanded` state — used by
Expand all / Collapse all menu items.

## Source
Lines 232–237 in `crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_navigation](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod.md) |
| called_by | [handle_project_tree_action](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handle_project_tree_action.md) |
