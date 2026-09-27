---
okf_version: "0.2"
type: Function
title: build_project_tree
description: Build the project tree from panel context data. Produces one root
resource: crates/oxide-app/src/panels/projects.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/projects/build_project_tree
language: rust
---

# build_project_tree

Build the project tree from panel context data. Produces one root

## Signature

```rust
pub fn build_project_tree(ctx: &PanelContext) -> Vec<TreeNode>
```

## Visibility

- `pub`

## Docstring

Build the project tree from panel context data. Produces one root
per loaded project so multi-project workspaces show all their
projects side by side. Single-project users see the same shape as
before (one root). `PanelContext::projects` is the source of truth;
the legacy `project_name` / `sheets` singletons are ignored here so
we never emit a duplicate root for the active project.

## Source
Lines 132–138 in `crates/oxide-app/src/panels/projects.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projects](/crates/oxide-app/src/panels/projects.md) |
| called_by | [handle_project_tree_action](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handle_project_tree_action.md) |
| called_by | [refresh_panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx/refresh_panel_ctx.md) |
