---
okf_version: "0.2"
type: Function
title: action_icon
description: Resolve the toolbar icon for the last-used action in a group.
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/action_icon
language: rust
---

# action_icon

Resolve the toolbar icon for the last-used action in a group.

## Signature

```rust
fn action_icon(action: &ActiveBarAction, tid: ThemeId) -> svg::Handle
```

## Docstring

Resolve the toolbar icon for the last-used action in a group.

## Source
Lines 370–473 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
| called_by | [view_bar](/crates/oxide-app/src/active_bar/mod/view_bar.md) |
