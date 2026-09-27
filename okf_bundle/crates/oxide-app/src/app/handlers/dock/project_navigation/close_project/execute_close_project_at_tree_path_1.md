---
okf_version: "0.2"
type: Function
title: execute_close_project_at_tree_path
description: Inner close-project flow that actually drops tabs + the
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/execute_close_project_at_tree_path_1
language: rust
---

# execute_close_project_at_tree_path

Inner close-project flow that actually drops tabs + the

## Signature

```rust
fn execute_close_project_at_tree_path(&mut self, tree_path: &[usize]) -> Task<Message>
```

## Docstring

Inner close-project flow that actually drops tabs + the
project entry. Called either directly (clean project) or via
the project-close confirm modal once the user picks Save All
or Discard All. Trusts that `dirty_paths` for this project is
already empty (callers ensure this).

## Source
Lines 71–133 in `crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_project](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.md) |
