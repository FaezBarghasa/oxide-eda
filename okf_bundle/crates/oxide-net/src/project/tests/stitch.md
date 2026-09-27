---
okf_version: "0.2"
type: Function
title: stitch
description: "Test-only bridge from the flat \"bare filename\" fixtures every test below"
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/stitch
language: rust
---

# stitch

Test-only bridge from the flat "bare filename" fixtures every test below

## Signature

```rust
fn stitch(
    root: &SchematicSheet,
    root_key: &str,
    children: &HashMap<String, SchematicSheet>,
) -> ProjectNetlist
```

## Docstring

Test-only bridge from the flat "bare filename" fixtures every test below
writes to the [`ProjectGraph`] shape #466 introduced: seeds `sheets` with
`root` under `root_key` plus every entry of `children`, then gives every
sheet an *identity* resolution submap — each of its own `ChildSheet.filename`s
maps to that same string as a [`SheetKey`]. That reproduces the flat,
single-namespace model these fixtures assume (a bare filename is globally
unique across the fixture), so only the call site changes, not the fixture
shape. The dedicated cross-directory tests below build a real
[`ProjectGraph`] by hand instead, precisely because that flat assumption is
what #466 stops the app from making.

## Source
Lines 213–219 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
| called_by | [assert_equiv](/crates/oxide-net/src/project/tests/assert_equiv.md) |
| called_by | [cycles_are_reported_without_hanging](/crates/oxide-net/src/project/tests/cycles_are_reported_without_hanging.md) |
| called_by | [duplicate_sibling_name_collision_is_suffixed](/crates/oxide-net/src/project/tests/duplicate_sibling_name_collision_is_suffixed.md) |
| called_by | [global_label_spans_two_sheets](/crates/oxide-net/src/project/tests/global_label_spans_two_sheets.md) |
| called_by | [local_net_labels_do_not_cross_sheets](/crates/oxide-net/src/project/tests/local_net_labels_do_not_cross_sheets.md) |
| called_by | [membership_aggregates_across_sheet_occurrences](/crates/oxide-net/src/project/tests/membership_aggregates_across_sheet_occurrences.md) |
| called_by | [missing_child_reported_and_local](/crates/oxide-net/src/project/tests/missing_child_reported_and_local.md) |
| called_by | [output_is_deterministic_across_map_order](/crates/oxide-net/src/project/tests/output_is_deterministic_across_map_order.md) |
| called_by | [power_symbol_and_power_label_merge_across_sheets](/crates/oxide-net/src/project/tests/power_symbol_and_power_label_merge_across_sheets.md) |
| called_by | [project_terminals_order_designators_naturally](/crates/oxide-net/src/project/tests/project_terminals_order_designators_naturally.md) |
| called_by | [same_child_instantiated_twice_is_not_shorted](/crates/oxide-net/src/project/tests/same_child_instantiated_twice_is_not_shorted.md) |
| called_by | [sheet_pin_anchors_to_wire_interior](/crates/oxide-net/src/project/tests/sheet_pin_anchors_to_wire_interior.md) |
| called_by | [sheet_pin_binds_global_child_label](/crates/oxide-net/src/project/tests/sheet_pin_binds_global_child_label.md) |
| called_by | [sheet_pin_binds_hierarchical_child_label](/crates/oxide-net/src/project/tests/sheet_pin_binds_hierarchical_child_label.md) |
| called_by | [single_sheet_name_collision_matches_build_netlist_and_is_reported](/crates/oxide-net/src/project/tests/single_sheet_name_collision_matches_build_netlist_and_is_reported.md) |
| called_by | [two_same_name_sheet_pins_merge_through_child](/crates/oxide-net/src/project/tests/two_same_name_sheet_pins_merge_through_child.md) |
| called_by | [unmatched_sheet_pin_stays_local](/crates/oxide-net/src/project/tests/unmatched_sheet_pin_stays_local.md) |
