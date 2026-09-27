---
okf_version: "0.2"
type: Function
title: project_tree_context_menu_overlay
description: Projects-panel tree right-click menu (+ its AddNewToProject
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/project_tree_context_menu_overlay
language: rust
---

# project_tree_context_menu_overlay

Projects-panel tree right-click menu (+ its AddNewToProject

## Signature

```rust
impl Oxide { pub(super) fn project_tree_context_menu_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(super)`

## Docstring

Projects-panel tree right-click menu (+ its AddNewToProject
submenu). Pushes dismiss, menu, then the optional submenu.

## Source
Lines 504–585 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |
