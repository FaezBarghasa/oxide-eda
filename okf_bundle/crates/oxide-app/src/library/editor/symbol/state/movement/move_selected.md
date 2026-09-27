---
okf_version: "0.2"
type: Function
title: move_selected
description: Move the currently-selected element to a new canvas position.
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/move_selected
language: rust
---

# move_selected

Move the currently-selected element to a new canvas position.

## Signature

```rust
pub fn move_selected(sym: &mut Symbol, sel: Option<SymbolSelection>, x: f64, y: f64)
```

## Visibility

- `pub`

## Docstring

Move the currently-selected element to a new canvas position.
Coordinates are in mm; callers should snap to the grid first.
For graphics this translates the entire shape so its anchor (TL
corner / `from` endpoint / `center` / `position`) lands on `(x, y)`.

## Source
Lines 10–29 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| calls | [translate_graphic_to](/crates/oxide-app/src/library/editor/symbol/state/rotation/translate_graphic_to.md) |
| called_by | [move_selected_translates_polygon_by_centroid_delta](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_translates_polygon_by_centroid_delta.md) |
| called_by | [move_selected_translates_rectangle_by_anchor_delta](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_translates_rectangle_by_anchor_delta.md) |
| called_by | [move_selected_updates_position](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_updates_position.md) |
| called_by | [apply_symbol_move](/crates/oxide-app/src/library/editor/symbol/updates/movement/apply_symbol_move.md) |
