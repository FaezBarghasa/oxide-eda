---
okf_version: "0.2"
type: Function
title: add_new_schematic
description: "`Add New ▸ Schematic` — Save-As dialog scoped to the project"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/add/add_new_schematic_1
language: rust
---

# add_new_schematic

`Add New ▸ Schematic` — Save-As dialog scoped to the project

## Signature

```rust
pub(crate) fn add_new_schematic(&mut self, tree_path: Vec<usize>) -> iced::Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

`Add New ▸ Schematic` — Save-As dialog scoped to the project
directory; result returns through [`ProjectMsg::AddNewSchematicPicked`].
The handler writes a blank `.snxsch`, registers the entry on
the project, and marks the .snxprj dirty.

## Source
Lines 51–78 in `crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [add](/crates/oxide-app/src/app/handlers/dock/project_navigation/add.md) |
| calls | [unique_name_in](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/unique_name_in.md) |
