---
okf_version: "0.2"
type: Function
title: add_project_footprint_library
description: "`Add New ▸ PCB Library` — counterpart to"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/add/add_project_footprint_library
language: rust
---

# add_project_footprint_library

`Add New ▸ PCB Library` — counterpart to

## Signature

```rust
impl Oxide { pub(crate) fn add_project_footprint_library(
        &mut self,
        tree_path: Vec<usize>,
    ) -> iced::Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

`Add New ▸ PCB Library` — counterpart to
[`add_project_symbol_library`] for `.snxfpt` files.

## Source
Lines 123–158 in `crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [add](/crates/oxide-app/src/app/handlers/dock/project_navigation/add.md) |
| calls | [unique_name_in](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/unique_name_in.md) |
