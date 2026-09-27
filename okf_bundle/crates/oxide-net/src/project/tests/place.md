---
okf_version: "0.2"
type: Function
title: place
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/place
language: rust
---

# place

## Signature

```rust
fn place(sheet: &mut SchematicSheet, reference: &str, lib_id: &str, origin: Point)
```

## Source
Lines 114–145 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| called_by | [duplicate_sibling_name_collision_is_suffixed](/crates/oxide-net/src/project/tests/duplicate_sibling_name_collision_is_suffixed.md) |
| called_by | [equivalence_gate_root_only](/crates/oxide-net/src/project/tests/equivalence_gate_root_only.md) |
| called_by | [global_label_spans_two_sheets](/crates/oxide-net/src/project/tests/global_label_spans_two_sheets.md) |
| called_by | [local_net_labels_do_not_cross_sheets](/crates/oxide-net/src/project/tests/local_net_labels_do_not_cross_sheets.md) |
| called_by | [membership_aggregates_across_sheet_occurrences](/crates/oxide-net/src/project/tests/membership_aggregates_across_sheet_occurrences.md) |
| called_by | [missing_child_reported_and_local](/crates/oxide-net/src/project/tests/missing_child_reported_and_local.md) |
| called_by | [a_flat_pages_missing_child_is_reported](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_missing_child_is_reported.md) |
| called_by | [a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root.md) |
| called_by | [a_page_listed_twice_still_contributes_one_occurrence](/crates/oxide-net/src/project/tests/multi_root/a_page_listed_twice_still_contributes_one_occurrence.md) |
| called_by | [a_page_referencing_the_project_root_does_not_stitch_the_root_twice](/crates/oxide-net/src/project/tests/multi_root/a_page_referencing_the_project_root_does_not_stitch_the_root_twice.md) |
| called_by | [flat_page_traversal_is_deterministic_across_map_insertion_order](/crates/oxide-net/src/project/tests/multi_root/flat_page_traversal_is_deterministic_across_map_insertion_order.md) |
| called_by | [flat_sibling_with_shared_global_label_merges_into_one_net](/crates/oxide-net/src/project/tests/multi_root/flat_sibling_with_shared_global_label_merges_into_one_net.md) |
| called_by | [flat_siblings_with_distinct_local_labels_stay_separate](/crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_distinct_local_labels_stay_separate.md) |
| called_by | [flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge](/crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_the_same_bare_local_name_collide_but_do_not_merge.md) |
| called_by | [stitch_referencing_page_last](/crates/oxide-net/src/project/tests/multi_root/stitch_referencing_page_last.md) |
| called_by | [two_flat_siblings_merge_by_shared_power_label_root_stays_separate](/crates/oxide-net/src/project/tests/multi_root/two_flat_siblings_merge_by_shared_power_label_root_stays_separate.md) |
| called_by | [output_is_deterministic_across_map_order](/crates/oxide-net/src/project/tests/output_is_deterministic_across_map_order.md) |
| called_by | [parent_child](/crates/oxide-net/src/project/tests/parent_child.md) |
| called_by | [place_power](/crates/oxide-net/src/project/tests/place_power.md) |
| called_by | [place_xform](/crates/oxide-net/src/project/tests/place_xform.md) |
| called_by | [power_symbol_and_power_label_merge_across_sheets](/crates/oxide-net/src/project/tests/power_symbol_and_power_label_merge_across_sheets.md) |
| called_by | [project_terminals_order_designators_naturally](/crates/oxide-net/src/project/tests/project_terminals_order_designators_naturally.md) |
| called_by | [same_child_instantiated_twice_is_not_shorted](/crates/oxide-net/src/project/tests/same_child_instantiated_twice_is_not_shorted.md) |
| called_by | [same_filename_children_of_different_parents_stitch_from_their_own_files](/crates/oxide-net/src/project/tests/same_filename_children_of_different_parents_stitch_from_their_own_files.md) |
| called_by | [sheet_pin_anchors_to_wire_interior](/crates/oxide-net/src/project/tests/sheet_pin_anchors_to_wire_interior.md) |
| called_by | [single_sheet_name_collision_matches_build_netlist_and_is_reported](/crates/oxide-net/src/project/tests/single_sheet_name_collision_matches_build_netlist_and_is_reported.md) |
| called_by | [two_same_name_sheet_pins_merge_through_child](/crates/oxide-net/src/project/tests/two_same_name_sheet_pins_merge_through_child.md) |
