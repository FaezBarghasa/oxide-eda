---
okf_version: "0.2"
type: Class
title: TreeNodeRole
description: "Role a project-tree node plays, which selects its right-click menu."
resource: crates/oxide-app/src/app/view/context_menu/project_tree.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/project_tree/TreeNodeRole
language: rust
---

# TreeNodeRole

Role a project-tree node plays, which selects its right-click menu.

## Signature

```rust
pub(super) enum TreeNodeRole
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub(super)`

## Docstring

Role a project-tree node plays, which selects its right-click menu.
Precedence matches the historic `if/else if` chain: a single-segment
path is always the project root, then a depth-3 `SnxLibrary` leaf, then
any file-backed openable leaf, then a container branch, else unknown.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 19–25 in `crates/oxide-app/src/app/view/context_menu/project_tree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_tree](/crates/oxide-app/src/app/view/context_menu/project_tree.md) |
