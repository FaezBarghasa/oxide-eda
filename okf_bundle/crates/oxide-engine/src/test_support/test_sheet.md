---
okf_version: "0.2"
type: Function
title: test_sheet
description: "An empty A4 sheet: no symbols, no wires, no graphics, no title block."
resource: crates/oxide-engine/src/test_support.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/test_support/test_sheet
language: rust
---

# test_sheet

An empty A4 sheet: no symbols, no wires, no graphics, no title block.

## Signature

```rust
pub(crate) fn test_sheet() -> SchematicSheet
```

## Visibility

- `pub(crate)`

## Docstring

An empty A4 sheet: no symbols, no wires, no graphics, no title block.

## Source
Lines 12–34 in `crates/oxide-engine/src/test_support.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [test_support](/crates/oxide-engine/src/test_support.md) |
| called_by | [engine](/crates/oxide-engine/src/exec/batch/engine.md) |
| called_by | [a_trunk_crossing_another_wires_interior_gets_no_junction](/crates/oxide-engine/src/lib/a_trunk_crossing_another_wires_interior_gets_no_junction.md) |
| called_by | [a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction](/crates/oxide-engine/src/lib/a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction.md) |
| called_by | [a_user_placed_junction_survives_reconcile_even_when_unjustified](/crates/oxide-engine/src/lib/a_user_placed_junction_survives_reconcile_even_when_unjustified.md) |
| called_by | [an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction](/crates/oxide-engine/src/lib/an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction.md) |
| called_by | [collect_exposed_sheet_ports_prefers_hierarchical_over_global](/crates/oxide-engine/src/lib/collect_exposed_sheet_ports_prefers_hierarchical_over_global.md) |
| called_by | [delete_child_sheet_and_undo](/crates/oxide-engine/src/lib/delete_child_sheet_and_undo.md) |
| called_by | [delete_selection_of_child_sheet_and_sheet_pin_are_undoable](/crates/oxide-engine/src/lib/delete_selection_of_child_sheet_and_sheet_pin_are_undoable.md) |
| called_by | [delete_selection_removes_bus_entry](/crates/oxide-engine/src/lib/delete_selection_removes_bus_entry.md) |
| called_by | [delete_selection_removes_child_sheet_and_its_pins](/crates/oxide-engine/src/lib/delete_selection_removes_child_sheet_and_its_pins.md) |
| called_by | [delete_selection_removes_only_the_targeted_sheet_pin](/crates/oxide-engine/src/lib/delete_selection_removes_only_the_targeted_sheet_pin.md) |
| called_by | [delete_sheet_pin_and_undo](/crates/oxide-engine/src/lib/delete_sheet_pin_and_undo.md) |
| called_by | [deleting_a_port_backed_sheet_pin_does_not_survive_reconcile](/crates/oxide-engine/src/lib/deleting_a_port_backed_sheet_pin_does_not_survive_reconcile.md) |
| called_by | [deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets](/crates/oxide-engine/src/lib/deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets.md) |
| called_by | [dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot](/crates/oxide-engine/src/lib/dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot.md) |
| called_by | [dragging_a_stub_across_a_trunk_gets_no_junction](/crates/oxide-engine/src/lib/dragging_a_stub_across_a_trunk_gets_no_junction.md) |
| called_by | [dragging_a_stub_onto_a_trunks_interior_gets_a_junction](/crates/oxide-engine/src/lib/dragging_a_stub_onto_a_trunks_interior_gets_a_junction.md) |
| called_by | [has_selected_items_recognizes_child_sheet_and_pin](/crates/oxide-engine/src/lib/has_selected_items_recognizes_child_sheet_and_pin.md) |
| called_by | [moving_sheet_pin_locks_to_nearest_sheet_edge](/crates/oxide-engine/src/lib/moving_sheet_pin_locks_to_nearest_sheet_edge.md) |
| called_by | [reconcile_child_sheet_pins_adds_new_and_removes_stale_auto_generated](/crates/oxide-engine/src/lib/reconcile_child_sheet_pins_adds_new_and_removes_stale_auto_generated.md) |
| called_by | [reconcile_preserves_position_for_user_moved_pin](/crates/oxide-engine/src/lib/reconcile_preserves_position_for_user_moved_pin.md) |
| called_by | [set_paper_size_persists_no_ops_and_undoes](/crates/oxide-engine/src/lib/set_paper_size_persists_no_ops_and_undoes.md) |
