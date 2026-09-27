---
okf_version: "0.2"
type: Function
title: project_root_node
description: "One project root — \"Source Documents\" / \"Libraries\" / \"Settings\"."
resource: crates/oxide-app/src/panels/projects.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/projects/project_root_node
language: rust
---

# project_root_node

One project root — "Source Documents" / "Libraries" / "Settings".

## Signature

```rust
fn project_root_node(project: &ProjectPanelInfo) -> TreeNode
```

## Docstring

One project root — "Source Documents" / "Libraries" / "Settings".
The Libraries branch lists this project's mounted `*.snxlib`
entries (right-click → Add New ▸ Component Library to add one);
it renders empty when the project has no libraries rather than
inheriting a workspace-wide symbol count.

## Source
Lines 145–282 in `crates/oxide-app/src/panels/projects.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [projects](/crates/oxide-app/src/panels/projects.md) |
| calls | [leaf](/crates/oxide-app/src/menu_bar/mod/leaf.md) |
| calls | [missing_label](/crates/oxide-app/src/panels/projects/missing_label.md) |
