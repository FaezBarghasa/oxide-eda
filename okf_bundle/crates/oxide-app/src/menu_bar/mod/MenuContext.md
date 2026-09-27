---
okf_version: "0.2"
type: Class
title: MenuContext
description: "Context passed into `view` so each menu leaf can decide whether to"
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/MenuContext
language: rust
---

# MenuContext

Context passed into `view` so each menu leaf can decide whether to

## Signature

```rust
pub struct MenuContext
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Context passed into `view` so each menu leaf can decide whether to
render as an active link or a disabled item. Keeps the menu
context-aware — e.g. Annotate / ERC / Save are unclickable when no
schematic is open.
[derive(Debug, Clone)]

## Methods

- `has_schematic`
- `has_pcb`
- `has_project`
- `has_selection`
- `can_undo`
- `can_redo`
- `has_symbol_editor`
- `has_footprint_editor`
- `scale_factor`
- `active_keymap`

## Source
Lines 211–238 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
