---
okf_version: "0.2"
type: Function
title: requires_net_color
description: "Whether `action` only makes sense when at least one net carries a"
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/requires_net_color
language: rust
---

# requires_net_color

Whether `action` only makes sense when at least one net carries a

## Signature

```rust
pub fn requires_net_color(action: &ActiveBarAction) -> bool
```

## Visibility

- `pub`

## Docstring

Whether `action` only makes sense when at least one net carries a
custom colour override. The Clear / Clear-All Net Color actions go
here; the seven NetColor pickers and Custom Color stay always-on
(they're the "arm" phase that paints colours onto nets).

## Source
Lines 54–57 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
| called_by | [action_enabled](/crates/oxide-app/src/active_bar/mod/action_enabled.md) |
