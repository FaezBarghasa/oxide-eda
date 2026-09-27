---
okf_version: "0.2"
type: Module
title: tests
description: Tests for project-level netlist stitching.
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests
language: rust
---

# tests

Tests for project-level netlist stitching.

## Docstring

Tests for project-level netlist stitching.

## Relationships

| Type | Target |
|------|--------|
| related | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| related | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| related | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| related | [junction](/crates/oxide-net/src/project/tests/junction.md) |
| related | [label](/crates/oxide-net/src/project/tests/label.md) |
| related | [lib_pin](/crates/oxide-net/src/project/tests/lib_pin.md) |
| related | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| related | [place](/crates/oxide-net/src/project/tests/place.md) |
| related | [place_power](/crates/oxide-net/src/project/tests/place_power.md) |
| related | [place_xform](/crates/oxide-net/src/project/tests/place_xform.md) |
| related | [sheet_pin](/crates/oxide-net/src/project/tests/sheet_pin.md) |
| related | [child_sheet](/crates/oxide-net/src/project/tests/child_sheet.md) |
| related | [names](/crates/oxide-net/src/project/tests/names.md) |
| related | [stitch](/crates/oxide-net/src/project/tests/stitch.md) |
| related | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
| related | [assert_equiv](/crates/oxide-net/src/project/tests/assert_equiv.md) |
| related | [equivalence_gate_root_only](/crates/oxide-net/src/project/tests/equivalence_gate_root_only.md) |
| related | [sheet_add_pin_lib](/crates/oxide-net/src/project/tests/sheet_add_pin_lib.md) |
| related | [parent_child](/crates/oxide-net/src/project/tests/parent_child.md) |
| related | [sheet_pin_binds_hierarchical_child_label](/crates/oxide-net/src/project/tests/sheet_pin_binds_hierarchical_child_label.md) |
| related | [sheet_pin_binds_global_child_label](/crates/oxide-net/src/project/tests/sheet_pin_binds_global_child_label.md) |
| related | [unmatched_sheet_pin_stays_local](/crates/oxide-net/src/project/tests/unmatched_sheet_pin_stays_local.md) |
| related | [global_label_spans_two_sheets](/crates/oxide-net/src/project/tests/global_label_spans_two_sheets.md) |
| related | [membership_aggregates_across_sheet_occurrences](/crates/oxide-net/src/project/tests/membership_aggregates_across_sheet_occurrences.md) |
| related | [power_symbol_and_power_label_merge_across_sheets](/crates/oxide-net/src/project/tests/power_symbol_and_power_label_merge_across_sheets.md) |
| related | [local_net_labels_do_not_cross_sheets](/crates/oxide-net/src/project/tests/local_net_labels_do_not_cross_sheets.md) |
| related | [same_child_instantiated_twice_is_not_shorted](/crates/oxide-net/src/project/tests/same_child_instantiated_twice_is_not_shorted.md) |
| related | [missing_child_reported_and_local](/crates/oxide-net/src/project/tests/missing_child_reported_and_local.md) |
| related | [cycles_are_reported_without_hanging](/crates/oxide-net/src/project/tests/cycles_are_reported_without_hanging.md) |
| related | [sheet_pin_anchors_to_wire_interior](/crates/oxide-net/src/project/tests/sheet_pin_anchors_to_wire_interior.md) |
| related | [two_same_name_sheet_pins_merge_through_child](/crates/oxide-net/src/project/tests/two_same_name_sheet_pins_merge_through_child.md) |
| related | [duplicate_sibling_name_collision_is_suffixed](/crates/oxide-net/src/project/tests/duplicate_sibling_name_collision_is_suffixed.md) |
| related | [single_sheet_name_collision_matches_build_netlist_and_is_reported](/crates/oxide-net/src/project/tests/single_sheet_name_collision_matches_build_netlist_and_is_reported.md) |
| related | [output_is_deterministic_across_map_order](/crates/oxide-net/src/project/tests/output_is_deterministic_across_map_order.md) |
| related | [project_terminals_order_designators_naturally](/crates/oxide-net/src/project/tests/project_terminals_order_designators_naturally.md) |
| related | [same_filename_children_of_different_parents_stitch_from_their_own_files](/crates/oxide-net/src/project/tests/same_filename_children_of_different_parents_stitch_from_their_own_files.md) |
