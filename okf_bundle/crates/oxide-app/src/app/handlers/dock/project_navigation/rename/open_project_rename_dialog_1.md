---
okf_version: "0.2"
type: Function
title: open_project_rename_dialog
description: "Open the rename modal seeded with the project name (the `.snxprj`"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/rename.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/rename/open_project_rename_dialog_1
language: rust
---

# open_project_rename_dialog

Open the rename modal seeded with the project name (the `.snxprj`

## Signature

```rust
pub(crate) fn open_project_rename_dialog(&mut self, tree_path: Vec<usize>)
```

## Visibility

- `pub(crate)`

## Docstring

Open the rename modal seeded with the project name (the `.snxprj`
file stem). On submit, [`handle_rename_submit`] sees
`is_project_rename = true` and renames the trio
`<old>.snxprj` / `<old>.snxsch` / `<old>.snxpcb` together.

## Source
Lines 32–52 in `crates/oxide-app/src/app/handlers/dock/project_navigation/rename.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rename](/crates/oxide-app/src/app/handlers/dock/project_navigation/rename.md) |
