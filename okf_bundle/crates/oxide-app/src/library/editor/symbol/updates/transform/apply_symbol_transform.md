---
okf_version: "0.2"
type: Function
title: apply_symbol_transform
resource: crates/oxide-app/src/library/editor/symbol/updates/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform
language: rust
---

# apply_symbol_transform

## Signature

```rust
pub(super) fn apply_symbol_transform(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 8–91 in `crates/oxide-app/src/library/editor/symbol/updates/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/symbol/updates/transform.md) |
| calls | [push_undo](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo.md) |
| calls | [rotate_pivot_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/rotate_pivot_msg_to_state.md) |
| calls | [rotate_selected_with_pivot](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_with_pivot.md) |
| calls | [selected_is_alignable](/crates/oxide-app/src/library/editor/symbol/state/mod/selected_is_alignable.md) |
| calls | [align_selected_to_grid](/crates/oxide-app/src/library/editor/symbol/state/movement/align_selected_to_grid.md) |
| calls | [push_undo_snapshot](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo_snapshot.md) |
| calls | [selected_is_deletable](/crates/oxide-app/src/library/editor/symbol/state/mod/selected_is_deletable.md) |
| calls | [close_pickers](/crates/oxide-app/src/library/editor/symbol/updates/mod/close_pickers.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [align_selected_to_grid_snaps_the_selected_pin_and_dirties_once](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_snaps_the_selected_pin_and_dirties_once.md) |
| called_by | [align_selected_to_grid_with_all_selection_on_empty_symbol_stays_clean](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_all_selection_on_empty_symbol_stays_clean.md) |
| called_by | [align_selected_to_grid_with_all_selection_snaps_every_pin](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_all_selection_snaps_every_pin.md) |
| called_by | [align_selected_to_grid_with_already_aligned_pin_stays_clean](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_already_aligned_pin_stays_clean.md) |
| called_by | [align_selected_to_grid_with_no_selection_stays_clean](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_no_selection_stays_clean.md) |
| called_by | [delete_selected_closes_open_fill_picker](/crates/oxide-app/src/library/editor/symbol/updates/transform/delete_selected_closes_open_fill_picker.md) |
