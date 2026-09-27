---
okf_version: "0.2"
type: Function
title: align_selected_to_grid_with_all_selection_on_empty_symbol_stays_clean
description: "#477 — `All` is alignable-shaped even on an empty symbol"
resource: crates/oxide-app/src/library/editor/symbol/updates/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_all_selection_on_empty_symbol_stays_clean
language: rust
---

# align_selected_to_grid_with_all_selection_on_empty_symbol_stays_clean

#477 — `All` is alignable-shaped even on an empty symbol

## Signature

```rust
fn align_selected_to_grid_with_all_selection_on_empty_symbol_stays_clean()
```

## Decorators

- `test`

## Docstring

#477 — `All` is alignable-shaped even on an empty symbol
(Ctrl+A with nothing placed yet), but `align_selected_to_grid`
snaps zero pins/graphics, so the handler must not leave a
spurious undo snapshot or dirty flag behind — and, the actual
bug, must not destroy the redo stack either. `push_undo` clears
`redo_snapshots` unconditionally; the earlier fix popped the
undo snapshot back off on a no-op but never restored
`redo_snapshots`, so an ordinary no-op Align-To-Grid click (e.g.
Ctrl+A on an empty symbol, or a selection already on-grid)
silently wiped the user's redo history. Seed one redo entry up
front and assert it survives the no-op.
[test]

## Source
Lines 204–224 in `crates/oxide-app/src/library/editor/symbol/updates/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/symbol/updates/transform.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/transform/new_editor.md) |
| calls | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
