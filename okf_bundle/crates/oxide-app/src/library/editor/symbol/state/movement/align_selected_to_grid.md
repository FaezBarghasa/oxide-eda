---
okf_version: "0.2"
type: Function
title: align_selected_to_grid
description: "Snap every pin/graphic named by `sel` onto the nearest multiple of"
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/align_selected_to_grid
language: rust
---

# align_selected_to_grid

Snap every pin/graphic named by `sel` onto the nearest multiple of

## Signature

```rust
pub fn align_selected_to_grid(
    sym: &mut Symbol,
    sel: &Option<SymbolSelection>,
    step_mm: f64,
) -> bool
```

## Visibility

- `pub`

## Docstring

Snap every pin/graphic named by `sel` onto the nearest multiple of
`step_mm`, in place. Mirrors the footprint editor's
`ActiveBarAlignSelectionToGrid` (#426): each element's own anchor
point(s) land on the grid independently — a `Rectangle`/`Line`
snaps `from` and `to` separately (so a shape already square with
the grid on one corner doesn't get skewed to keep it), `Circle`/
`Arc` snap `center` only (radius untouched), `Text` snaps
`position`, and `Polygon` snaps every vertex. Returns `true` only
when a snap actually moved a coordinate — a delta check against the
pre-snap value, not merely "the selection resolved to an existing
pin/graphic" (#477: a pin or shape already sitting exactly on the
grid, or an `All`/`Multiple` selection made entirely of such
elements, must report `changed = false` so the caller's undo/redo
gate treats it as the no-op it is). See [`super::selected_is_alignable`]
for the selection-kind precheck most callers should run first.

## Source
Lines 85–145 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| calls | [snap_pin_to_grid](/crates/oxide-app/src/library/editor/symbol/state/movement/snap_pin_to_grid.md) |
| calls | [snap_graphic_to_grid](/crates/oxide-app/src/library/editor/symbol/state/movement/snap_graphic_to_grid.md) |
| called_by | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
