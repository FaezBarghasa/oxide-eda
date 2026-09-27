---
okf_version: "0.2"
type: Function
title: handled_tree_message_returns_some_task
description: "Regression (#99 part 1): `handle_dock_project_navigation_panel_message`"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handled_tree_message_returns_some_task
language: rust
---

# handled_tree_message_returns_some_task

Regression (#99 part 1): `handle_dock_project_navigation_panel_message`

## Signature

```rust
fn handled_tree_message_returns_some_task()
```

## Decorators

- `test`

## Docstring

Regression (#99 part 1): `handle_dock_project_navigation_panel_message`
used to return `bool` and its `TreeMsg::Select` double-click arm
discarded `open_project_tree_document`'s `Task` via `let _ = ...`.
The explicit `Option<Task<Message>>` annotation is a compile-time
tripwire — this test stops compiling if the signature regresses
back to `bool`.
[test]

## Source
Lines 308–318 in `crates/oxide-app/src/app/handlers/dock/project_navigation/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_navigation](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod.md) |
