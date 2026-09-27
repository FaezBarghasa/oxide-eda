---
okf_version: "0.2"
type: Function
title: add_existing_to_project
description: "`Add Existing to Project…` — open a multi-select file picker"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/add/add_existing_to_project
language: rust
---

# add_existing_to_project

`Add Existing to Project…` — open a multi-select file picker

## Signature

```rust
impl Oxide { pub(crate) fn add_existing_to_project(&mut self, tree_path: Vec<usize>) -> iced::Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

`Add Existing to Project…` — open a multi-select file picker
scoped to schematic / PCB / library extensions. Picked paths
land in [`ProjectMsg::AddExistingFilePicked`]; the handler copies
any outside the project directory in turn and opens each.

## Source
Lines 14–45 in `crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [add](/crates/oxide-app/src/app/handlers/dock/project_navigation/add.md) |
