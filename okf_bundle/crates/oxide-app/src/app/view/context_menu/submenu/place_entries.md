---
okf_version: "0.2"
type: Function
title: place_entries
description: "Place submenu — wires, buses, ports, power, directives, harness, sheet"
resource: crates/oxide-app/src/app/view/context_menu/submenu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/submenu/place_entries
language: rust
---

# place_entries

Place submenu — wires, buses, ports, power, directives, harness, sheet

## Signature

```rust
pub(super) fn place_entries(tid: ThemeId) -> Vec<DropdownEntry<Message>>
```

## Visibility

- `pub(super)`

## Docstring

Place submenu — wires, buses, ports, power, directives, harness, sheet
symbols, component, and text. Every row is always enabled.

## Source
Lines 20–114 in `crates/oxide-app/src/app/view/context_menu/submenu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [submenu](/crates/oxide-app/src/app/view/context_menu/submenu.md) |
| calls | [dd_kb](/crates/oxide-app/src/app/view/context_menu/items/dd_kb.md) |
| called_by | [view_context_submenu](/crates/oxide-app/src/app/view/context_menu/submenu/view_context_submenu.md) |
