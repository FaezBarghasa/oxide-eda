---
okf_version: "0.2"
type: Function
title: id_for_action
description: "The command id naming an action, if it has one."
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/id_for_action
language: rust
---

# id_for_action

The command id naming an action, if it has one.

## Signature

```rust
pub(crate) fn id_for_action(action: &ActiveBarAction) -> Option<&'static str>
```

## Visibility

- `pub(crate)`

## Docstring

The command id naming an action, if it has one.

The reverse of [`action_for_id`], for view code that holds an action
and wants the catalog entry behind it — the Active Bar renders its
row labels through this so the visible text lives in the command
table rather than in the view (#271).

## Source
Lines 147–152 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [action_label](/crates/oxide-app/src/app/command/active_bar/action_label.md) |
