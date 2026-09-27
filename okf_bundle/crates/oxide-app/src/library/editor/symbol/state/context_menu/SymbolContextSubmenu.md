---
okf_version: "0.2"
type: Class
title: SymbolContextSubmenu
description: The one submenu the symbol context menu currently has (Place ▸).
resource: crates/oxide-app/src/library/editor/symbol/state/context_menu.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/context_menu/SymbolContextSubmenu
language: rust
---

# SymbolContextSubmenu

The one submenu the symbol context menu currently has (Place ▸).

## Signature

```rust
pub enum SymbolContextSubmenu
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

The one submenu the symbol context menu currently has (Place ▸).
Kept as an enum (not a bare `bool`) so a second submenu can slot in
later without reshaping `SymbolContextMenuState`.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 33–35 in `crates/oxide-app/src/library/editor/symbol/state/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/state/context_menu.md) |
