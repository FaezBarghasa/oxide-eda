---
okf_version: "0.2"
type: Function
title: cmd_label
description: "Menu-display label for `command_id`, sourced from the command table's"
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/cmd_label
language: rust
---

# cmd_label

Menu-display label for `command_id`, sourced from the command table's

## Signature

```rust
fn cmd_label(command_id: &str, fallback: &str) -> String
```

## Docstring

Menu-display label for `command_id`, sourced from the command table's
terse `menu_label` (which falls back to the descriptive `label`). The
`fallback` literal covers a command with no catalog entry so the visible
menu text is never changed by the lookup. Pairs with [`shortcut_for`] so
a menu row's label and keybinding come from the same `AppCommandId`.

## Source
Lines 339–345 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| calls | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
