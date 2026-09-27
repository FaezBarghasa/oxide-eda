---
okf_version: "0.2"
type: Class
title: SymbolContextMenuState
description: "`(x, y)` are **window-absolute** screen coords (already include"
resource: crates/oxide-app/src/library/editor/symbol/state/context_menu.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/context_menu/SymbolContextMenuState
language: rust
---

# SymbolContextMenuState

`(x, y)` are **window-absolute** screen coords (already include

## Signature

```rust
pub struct SymbolContextMenuState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

`(x, y)` are **window-absolute** screen coords (already include
menu-bar + tab-bar offsets — computed in `canvas::input::pointer`'s
`on_secondary_release` from `bounds.x + cursor.x`). `target` records
what the cursor was over at right-click time so the update layer can
select-first (Altium parity) before opening the menu; `open_submenu`
tracks which submenu row (if any) is accordion-expanded in place.
[derive(Debug, Clone)]

## Methods

- `x`
- `y`
- `target`
- `open_submenu`

## Source
Lines 12–17 in `crates/oxide-app/src/library/editor/symbol/state/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/state/context_menu.md) |
