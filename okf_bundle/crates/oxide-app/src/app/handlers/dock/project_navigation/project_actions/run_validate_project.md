---
okf_version: "0.2"
type: Function
title: run_validate_project
description: "Validate Project — promote the project to active, ensure its"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/project_actions.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/project_actions/run_validate_project
language: rust
---

# run_validate_project

Validate Project — promote the project to active, ensure its

## Signature

```rust
impl Oxide { pub(crate) fn run_validate_project(&mut self, tree_path: Vec<usize>) -> iced::Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Validate Project — promote the project to active, ensure its
root schematic is open, then dispatch the existing ERC dialog.
`tree_path[0]` is the owning project; we open the schematic
root if no tab from this project is currently active so the
ERC engine targets the right sheet.

## Source
Lines 32–74 in `crates/oxide-app/src/app/handlers/dock/project_navigation/project_actions.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_actions](/crates/oxide-app/src/app/handlers/dock/project_navigation/project_actions.md) |
