---
okf_version: "0.2"
type: Module
title: to_lib_symbol_tests
description: "Tests for `Symbol::to_lib_symbol` (issue #365, part 1 of 2)."
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests
language: rust
---

# to_lib_symbol_tests

Tests for `Symbol::to_lib_symbol` (issue #365, part 1 of 2).

## Docstring

Tests for `Symbol::to_lib_symbol` (issue #365, part 1 of 2).

Sibling of `to_lib_symbol`, not a child module of it — same
constraint `chain_tests.rs` documents for `chain`: only
`to_lib_symbol`'s public surface (`Symbol::to_lib_symbol` itself) is
exercised here, exactly what a real caller has.

## Relationships

| Type | Target |
|------|--------|
| related | [pin](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin.md) |
| related | [line_graphic](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/line_graphic.md) |
| related | [approx](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/approx.md) |
| related | [two_unit_symbol_graphics_and_part_zero_pins_carry_the_right_unit](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/two_unit_symbol_graphics_and_part_zero_pins_carry_the_right_unit.md) |
| related | [positions_carry_over_unchanged_no_y_flip](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/positions_carry_over_unchanged_no_y_flip.md) |
| related | [pin_orientation_maps_to_rotation_degrees_for_all_four_variants](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_orientation_maps_to_rotation_degrees_for_all_four_variants.md) |
| related | [pin_rotation_matches_oxide_output_pin_direction_convention](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_rotation_matches_oxide_output_pin_direction_convention.md) |
| related | [pin_direction_convention](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_direction_convention.md) |
| related | [pin_direction_mapping_is_total_for_every_source_variant](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_direction_mapping_is_total_for_every_source_variant.md) |
| related | [fill_none_and_some_map_to_fill_type](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/fill_none_and_some_map_to_fill_type.md) |
| related | [header_fields_map_designator_comment_description_and_caller_supplied_id](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/header_fields_map_designator_comment_description_and_caller_supplied_id.md) |
| related | [pin_shape_style_uses_outside_edge_symbol_as_the_authoritative_slot](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_shape_style_uses_outside_edge_symbol_as_the_authoritative_slot.md) |
| related | [arc_converts_to_three_points_along_the_ccw_sweep](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/arc_converts_to_three_points_along_the_ccw_sweep.md) |
| related | [polygon_closes_explicitly_when_converted_to_polyline](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/polygon_closes_explicitly_when_converted_to_polyline.md) |
