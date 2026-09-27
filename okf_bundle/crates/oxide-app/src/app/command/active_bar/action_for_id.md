---
okf_version: "0.2"
type: Function
title: action_for_id
description: "The action a command id names, if it is an Active Bar command."
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/action_for_id
language: rust
---

# action_for_id

The action a command id names, if it is an Active Bar command.

## Signature

```rust
pub(crate) fn action_for_id(id: &str) -> Option<ActiveBarAction>
```

## Visibility

- `pub(crate)`

## Docstring

The action a command id names, if it is an Active Bar command.

Clones rather than copies: `ActiveBarAction` is not `Copy`, and a
clone here is a keystroke-scale cost.

## Source
Lines 134–139 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [resolve](/crates/oxide-app/src/app/command/bridge/resolve.md) |
