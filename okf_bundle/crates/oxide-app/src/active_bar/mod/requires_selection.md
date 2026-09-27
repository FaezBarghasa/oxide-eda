---
okf_version: "0.2"
type: Function
title: requires_selection
description: "Whether `action` needs at least one selected item to make sense."
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/requires_selection
language: rust
---

# requires_selection

Whether `action` needs at least one selected item to make sense.

## Signature

```rust
pub fn requires_selection(action: &ActiveBarAction) -> bool
```

## Visibility

- `pub`

## Docstring

Whether `action` needs at least one selected item to make sense.
Transform / align / distribute family — Altium greys these out when
the selection is empty. Net-colour picks are excluded because the
NetColor flow is "arm-then-apply", not "act on selection".

## Source
Lines 22–48 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
| called_by | [action_enabled](/crates/oxide-app/src/active_bar/mod/action_enabled.md) |
