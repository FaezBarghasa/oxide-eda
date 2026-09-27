# to_lib_symbol_tests

## Functions

- [approx](approx.md)
- [arc_converts_to_three_points_along_the_ccw_sweep](arc_converts_to_three_points_along_the_ccw_sweep.md) — `SymbolGraphicKind::Arc` (`center`/`radius`/CCW `start_deg..end_deg`)
- [fill_none_and_some_map_to_fill_type](fill_none_and_some_map_to_fill_type.md) — Fill is lossy by construction: `None -> FillType::None`, `Some(_) ->
- [header_fields_map_designator_comment_description_and_caller_supplied_id](header_fields_map_designator_comment_description_and_caller_supplied_id.md) — Header fields: `designator -> reference`, `comment -> value`,
- [line_graphic](line_graphic.md)
- [pin](pin.md)
- [pin_direction_convention](pin_direction_convention.md) — Mirrors `crates/oxide-output/src/svg/symbols.rs`'s `pin_direction`
- [pin_direction_mapping_is_total_for_every_source_variant](pin_direction_mapping_is_total_for_every_source_variant.md) — Acceptance criterion: pin the complete `PinDirection` mapping (every
- [pin_orientation_maps_to_rotation_degrees_for_all_four_variants](pin_orientation_maps_to_rotation_degrees_for_all_four_variants.md) — Acceptance criterion: pin the complete `PinOrientation` -> rotation
- [pin_rotation_matches_oxide_output_pin_direction_convention](pin_rotation_matches_oxide_output_pin_direction_convention.md) — Ground-truth convention test: ties `pin_rotation_deg`'s output to the
- [pin_shape_style_uses_outside_edge_symbol_as_the_authoritative_slot](pin_shape_style_uses_outside_edge_symbol_as_the_authoritative_slot.md) — `outside_edge_symbol` is the chosen authoritative glyph slot; the
- [polygon_closes_explicitly_when_converted_to_polyline](polygon_closes_explicitly_when_converted_to_polyline.md) — `SymbolGraphicKind::Polygon`'s vertex ring is closed implicitly; the
- [positions_carry_over_unchanged_no_y_flip](positions_carry_over_unchanged_no_y_flip.md) — Acceptance criterion: pin and graphic positions are byte-identical
- [two_unit_symbol_graphics_and_part_zero_pins_carry_the_right_unit](two_unit_symbol_graphics_and_part_zero_pins_carry_the_right_unit.md) — Acceptance criterion: a two-unit symbol whose part-1 and part-2
