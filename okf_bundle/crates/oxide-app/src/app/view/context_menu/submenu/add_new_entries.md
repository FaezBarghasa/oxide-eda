---
okf_version: "0.2"
type: Function
title: add_new_entries
description: "Add New to Project submenu — the master \"Add New\" picker for the"
resource: crates/oxide-app/src/app/view/context_menu/submenu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/submenu/add_new_entries
language: rust
---

# add_new_entries

Add New to Project submenu — the master "Add New" picker for the

## Signature

```rust
pub(super) fn add_new_entries(tid: ThemeId, target: Vec<usize>) -> Vec<DropdownEntry<Message>>
```

## Visibility

- `pub(super)`

## Docstring

Add New to Project submenu — the master "Add New" picker for the
right-clicked project (resolved as `target`). Schematic / Component
Library / Symbol Library are wired; the rest stay version-badged stubs
until their editors land.

## Source
Lines 196–267 in `crates/oxide-app/src/app/view/context_menu/submenu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [submenu](/crates/oxide-app/src/app/view/context_menu/submenu.md) |
| calls | [dd_msg](/crates/oxide-app/src/app/view/context_menu/items/dd_msg.md) |
| calls | [ProjectTreeAction](/crates/oxide-app/src/app/contracts/state/ProjectTreeAction.md) |
| calls | [dd_disabled](/crates/oxide-app/src/app/view/context_menu/items/dd_disabled.md) |
| called_by | [view_context_submenu](/crates/oxide-app/src/app/view/context_menu/submenu/view_context_submenu.md) |
