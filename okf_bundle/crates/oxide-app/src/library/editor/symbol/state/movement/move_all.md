---
okf_version: "0.2"
type: Function
title: move_all
description: "Shift every pin and every graphic by `(dx, dy)` mm."
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/move_all
language: rust
---

# move_all

Shift every pin and every graphic by `(dx, dy)` mm.

## Signature

```rust
pub fn move_all(sym: &mut Symbol, dx: f64, dy: f64)
```

## Visibility

- `pub`

## Docstring

Shift every pin and every graphic by `(dx, dy)` mm.

Used when the user drags with `SymbolSelection::All` active (Ctrl+A
select-all). The caller is responsible for computing the delta and
for grid-snapping if desired.

## Source
Lines 36–44 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| calls | [translate_graphic_by](/crates/oxide-app/src/library/editor/symbol/state/rotation/translate_graphic_by.md) |
| called_by | [apply_symbol_move](/crates/oxide-app/src/library/editor/symbol/updates/movement/apply_symbol_move.md) |
