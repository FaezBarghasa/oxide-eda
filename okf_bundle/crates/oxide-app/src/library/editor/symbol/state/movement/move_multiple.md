---
okf_version: "0.2"
type: Function
title: move_multiple
description: "Shift only the specified pins and graphics by `(dx, dy)` mm."
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/move_multiple
language: rust
---

# move_multiple

Shift only the specified pins and graphics by `(dx, dy)` mm.

## Signature

```rust
pub fn move_multiple(
    sym: &mut Symbol,
    pin_indices: &[usize],
    graphic_indices: &[usize],
    dx: f64,
    dy: f64,
)
```

## Visibility

- `pub`

## Docstring

Shift only the specified pins and graphics by `(dx, dy)` mm.

Used when the user drags with `SymbolSelection::Multiple` active
(box selection result). Out-of-range indices are silently skipped.

## Source
Lines 50–68 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| calls | [translate_graphic_by](/crates/oxide-app/src/library/editor/symbol/state/rotation/translate_graphic_by.md) |
| called_by | [apply_symbol_move](/crates/oxide-app/src/library/editor/symbol/updates/movement/apply_symbol_move.md) |
