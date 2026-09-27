---
okf_version: "0.2"
type: Function
title: child_sheet
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/child_sheet
language: rust
---

# child_sheet

## Signature

```rust
fn child_sheet(name: &str, filename: &str, pins: Vec<SheetPin>) -> ChildSheet
```

## Source
Lines 182–197 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| called_by | [cycles_are_reported_without_hanging](/crates/oxide-net/src/project/tests/cycles_are_reported_without_hanging.md) |
| called_by | [duplicate_sibling_name_collision_is_suffixed](/crates/oxide-net/src/project/tests/duplicate_sibling_name_collision_is_suffixed.md) |
| called_by | [global_label_spans_two_sheets](/crates/oxide-net/src/project/tests/global_label_spans_two_sheets.md) |
| called_by | [local_net_labels_do_not_cross_sheets](/crates/oxide-net/src/project/tests/local_net_labels_do_not_cross_sheets.md) |
| called_by | [membership_aggregates_across_sheet_occurrences](/crates/oxide-net/src/project/tests/membership_aggregates_across_sheet_occurrences.md) |
| called_by | [missing_child_reported_and_local](/crates/oxide-net/src/project/tests/missing_child_reported_and_local.md) |
| called_by | [a_flat_pages_missing_child_is_reported](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_missing_child_is_reported.md) |
| called_by | [a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root](/crates/oxide-net/src/project/tests/multi_root/a_flat_pages_own_child_sheet_is_visited_once_not_walked_again_as_a_root.md) |
| called_by | [a_page_referencing_the_project_root_does_not_stitch_the_root_twice](/crates/oxide-net/src/project/tests/multi_root/a_page_referencing_the_project_root_does_not_stitch_the_root_twice.md) |
| called_by | [stitch_referencing_page_last](/crates/oxide-net/src/project/tests/multi_root/stitch_referencing_page_last.md) |
| called_by | [two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang](/crates/oxide-net/src/project/tests/multi_root/two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang.md) |
| called_by | [output_is_deterministic_across_map_order](/crates/oxide-net/src/project/tests/output_is_deterministic_across_map_order.md) |
| called_by | [parent_child](/crates/oxide-net/src/project/tests/parent_child.md) |
| called_by | [power_symbol_and_power_label_merge_across_sheets](/crates/oxide-net/src/project/tests/power_symbol_and_power_label_merge_across_sheets.md) |
| called_by | [same_child_instantiated_twice_is_not_shorted](/crates/oxide-net/src/project/tests/same_child_instantiated_twice_is_not_shorted.md) |
| called_by | [same_filename_children_of_different_parents_stitch_from_their_own_files](/crates/oxide-net/src/project/tests/same_filename_children_of_different_parents_stitch_from_their_own_files.md) |
| called_by | [sheet_pin_anchors_to_wire_interior](/crates/oxide-net/src/project/tests/sheet_pin_anchors_to_wire_interior.md) |
| called_by | [two_same_name_sheet_pins_merge_through_child](/crates/oxide-net/src/project/tests/two_same_name_sheet_pins_merge_through_child.md) |
