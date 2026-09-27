---
okf_version: "0.2"
type: Class
title: SymbolMenuRow
description: "One row of the symbol context menu. `id` is a stable, kebab-case,"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/SymbolMenuRow
language: rust
---

# SymbolMenuRow

One row of the symbol context menu. `id` is a stable, kebab-case,

## Signature

```rust
pub struct SymbolMenuRow
```

## Visibility

- `pub`

## Docstring

One row of the symbol context menu. `id` is a stable, kebab-case,
`symbol.`-namespaced command id (the command-registry epic will
index these — see `menu_bar` / keymap catalog for the sibling
convention). `msg` is the message this row fires on click; `None`
for a submenu header, which instead toggles `submenu`'s visibility.
`submenu`, when present, holds the rows shown in place directly
below this row while it's the open submenu (accordion — mirrors
the footprint context menu's Place ▸ / Selection ▸ / View ▸
expand-in-place behaviour), not a hover flyout.

## Methods

- `id`
- `label`
- `enabled`
- `msg`
- `submenu`

## Source
Lines 27–33 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
