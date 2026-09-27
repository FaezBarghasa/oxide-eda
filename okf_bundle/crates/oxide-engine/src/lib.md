---
okf_version: "0.2"
type: Module
title: lib
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib
language: rust
---

# lib

## Relationships

| Type | Target |
|------|--------|
| related | [Engine](/crates/oxide-engine/src/lib/Engine.md) |
| related | [new](/crates/oxide-engine/src/lib/new.md) |
| related | [new_with_path](/crates/oxide-engine/src/lib/new_with_path.md) |
| related | [open](/crates/oxide-engine/src/lib/open.md) |
| related | [save](/crates/oxide-engine/src/lib/save.md) |
| related | [save_as](/crates/oxide-engine/src/lib/save_as.md) |
| related | [execute](/crates/oxide-engine/src/lib/execute.md) |
| related | [document](/crates/oxide-engine/src/lib/document.md) |
| related | [path](/crates/oxide-engine/src/lib/path.md) |
| related | [set_path](/crates/oxide-engine/src/lib/set_path.md) |
| related | [set_document](/crates/oxide-engine/src/lib/set_document.md) |
| related | [new](/crates/oxide-engine/src/lib/new.md) |
| related | [new_with_path](/crates/oxide-engine/src/lib/new_with_path.md) |
| related | [open](/crates/oxide-engine/src/lib/open.md) |
| related | [save](/crates/oxide-engine/src/lib/save.md) |
| related | [save_as](/crates/oxide-engine/src/lib/save_as.md) |
| related | [execute](/crates/oxide-engine/src/lib/execute.md) |
| related | [document](/crates/oxide-engine/src/lib/document.md) |
| related | [path](/crates/oxide-engine/src/lib/path.md) |
| related | [set_path](/crates/oxide-engine/src/lib/set_path.md) |
| related | [set_document](/crates/oxide-engine/src/lib/set_document.md) |
| related | [collect_exposed_sheet_ports_prefers_hierarchical_over_global](/crates/oxide-engine/src/lib/collect_exposed_sheet_ports_prefers_hierarchical_over_global.md) |
| related | [reconcile_child_sheet_pins_adds_new_and_removes_stale_auto_generated](/crates/oxide-engine/src/lib/reconcile_child_sheet_pins_adds_new_and_removes_stale_auto_generated.md) |
| related | [reconcile_preserves_position_for_user_moved_pin](/crates/oxide-engine/src/lib/reconcile_preserves_position_for_user_moved_pin.md) |
| related | [moving_sheet_pin_locks_to_nearest_sheet_edge](/crates/oxide-engine/src/lib/moving_sheet_pin_locks_to_nearest_sheet_edge.md) |
| related | [test_sheet_pin](/crates/oxide-engine/src/lib/test_sheet_pin.md) |
| related | [test_child_sheet](/crates/oxide-engine/src/lib/test_child_sheet.md) |
| related | [delete_selection_removes_child_sheet_and_its_pins](/crates/oxide-engine/src/lib/delete_selection_removes_child_sheet_and_its_pins.md) |
| related | [delete_selection_removes_only_the_targeted_sheet_pin](/crates/oxide-engine/src/lib/delete_selection_removes_only_the_targeted_sheet_pin.md) |
| related | [delete_selection_of_child_sheet_and_sheet_pin_are_undoable](/crates/oxide-engine/src/lib/delete_selection_of_child_sheet_and_sheet_pin_are_undoable.md) |
| related | [has_selected_items_recognizes_child_sheet_and_pin](/crates/oxide-engine/src/lib/has_selected_items_recognizes_child_sheet_and_pin.md) |
| related | [delete_selection_removes_bus_entry](/crates/oxide-engine/src/lib/delete_selection_removes_bus_entry.md) |
| related | [partition_cuttable_keeps_child_sheet_and_pin_out_of_cut](/crates/oxide-engine/src/lib/partition_cuttable_keeps_child_sheet_and_pin_out_of_cut.md) |
| related | [deleting_a_port_backed_sheet_pin_does_not_survive_reconcile](/crates/oxide-engine/src/lib/deleting_a_port_backed_sheet_pin_does_not_survive_reconcile.md) |
| related | [set_paper_size_persists_no_ops_and_undoes](/crates/oxide-engine/src/lib/set_paper_size_persists_no_ops_and_undoes.md) |
| related | [wire](/crates/oxide-engine/src/lib/wire.md) |
| related | [a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction](/crates/oxide-engine/src/lib/a_trunk_drawn_through_an_existing_wires_endpoint_gets_a_junction.md) |
| related | [a_trunk_crossing_another_wires_interior_gets_no_junction](/crates/oxide-engine/src/lib/a_trunk_crossing_another_wires_interior_gets_no_junction.md) |
| related | [dragging_a_stub_onto_a_trunks_interior_gets_a_junction](/crates/oxide-engine/src/lib/dragging_a_stub_onto_a_trunks_interior_gets_a_junction.md) |
| related | [dragging_a_stub_across_a_trunk_gets_no_junction](/crates/oxide-engine/src/lib/dragging_a_stub_across_a_trunk_gets_no_junction.md) |
| related | [an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction](/crates/oxide-engine/src/lib/an_endpoint_off_the_trunk_in_key_space_gets_no_lying_junction.md) |
| related | [dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot](/crates/oxide-engine/src/lib/dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot.md) |
| related | [a_user_placed_junction_survives_reconcile_even_when_unjustified](/crates/oxide-engine/src/lib/a_user_placed_junction_survives_reconcile_even_when_unjustified.md) |
| related | [deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets](/crates/oxide-engine/src/lib/deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets.md) |
| related | [delete_child_sheet_and_undo](/crates/oxide-engine/src/lib/delete_child_sheet_and_undo.md) |
| related | [delete_sheet_pin_and_undo](/crates/oxide-engine/src/lib/delete_sheet_pin_and_undo.md) |
