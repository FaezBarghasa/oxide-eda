---
okf_version: "0.2"
type: Function
title: add_pin
description: Add a pin at the given canvas coordinates and return its index in
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/add_pin
language: rust
---

# add_pin

Add a pin at the given canvas coordinates and return its index in

## Signature

```rust
pub fn add_pin(sym: &mut Symbol, x: f64, y: f64, part_number: u8) -> usize
```

## Visibility

- `pub`

## Docstring

Add a pin at the given canvas coordinates and return its index in
`Symbol::pins`. Auto-assigns the next free numeric pin number and
scopes it to `part_number` (typically the editor's active sub-part
for multi-part components; `1` for single-part).

## Source
Lines 545–553 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| calls | [next_pin_number](/crates/oxide-app/src/library/editor/symbol/state/mod/next_pin_number.md) |
| called_by | [add_pin_assigns_next_number](/crates/oxide-app/src/library/editor/symbol/state/tests/add_pin_assigns_next_number.md) |
| called_by | [add_pin_records_active_part](/crates/oxide-app/src/library/editor/symbol/state/tests/add_pin_records_active_part.md) |
| called_by | [delete_pin_clears_selection_via_return](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_pin_clears_selection_via_return.md) |
| called_by | [delete_unit_out_of_range_leaves_count_unchanged](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_out_of_range_leaves_count_unchanged.md) |
| called_by | [delete_unit_removes_and_renumbers](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_unit_removes_and_renumbers.md) |
| called_by | [hit_test_ignores_other_unit_pin](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_ignores_other_unit_pin.md) |
| called_by | [hit_test_returns_pin](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_returns_pin.md) |
| called_by | [max_part_number_ignores_part_zero](/crates/oxide-app/src/library/editor/symbol/state/tests/max_part_number_ignores_part_zero.md) |
| called_by | [move_selected_updates_position](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_updates_position.md) |
| called_by | [rotate_selected_rotates_pin_orientation_in_place](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_rotates_pin_orientation_in_place.md) |
| called_by | [select_in_box_all_uses_visible_counts](/crates/oxide-app/src/library/editor/symbol/state/tests/select_in_box_all_uses_visible_counts.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [align_selected_to_grid_snaps_the_selected_pin_and_dirties_once](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_snaps_the_selected_pin_and_dirties_once.md) |
| called_by | [align_selected_to_grid_with_all_selection_snaps_every_pin](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_all_selection_snaps_every_pin.md) |
| called_by | [align_selected_to_grid_with_already_aligned_pin_stays_clean](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_already_aligned_pin_stays_clean.md) |
