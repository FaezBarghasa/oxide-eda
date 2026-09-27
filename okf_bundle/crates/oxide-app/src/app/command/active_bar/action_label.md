---
okf_version: "0.2"
type: Function
title: action_label
description: "The label a surface should show for an action: the catalog's"
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/action_label
language: rust
---

# action_label

The label a surface should show for an action: the catalog's

## Signature

```rust
pub(crate) fn action_label(action: &ActiveBarAction, fallback: &'static str) -> &'static str
```

## Visibility

- `pub(crate)`

## Docstring

The label a surface should show for an action: the catalog's
`menu_label` when the action has an id, else `fallback`.

The Active Bar's row text now lives in the command table (#271), so a
menu, the palette and the shortcuts pane can render the same command
without a second copy of its wording. `fallback` covers the actions
still without an id — the parameterised families awaiting
`CommandArgs` — so no row can lose its label.

## Source
Lines 191–203 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| calls | [variant_name](/crates/oxide-app/src/app/command/active_bar/variant_name.md) |
| calls | [id_for_action](/crates/oxide-app/src/app/command/active_bar/id_for_action.md) |
| calls | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
| called_by | [dd_item](/crates/oxide-app/src/active_bar/dropdown/dd_item.md) |
