---
okf_version: "0.2"
type: Module
title: tests
description: Unit tests for the symbol primitive + pin TSV codec.
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests
language: rust
---

# tests

Unit tests for the symbol primitive + pin TSV codec.

## Docstring

Unit tests for the symbol primitive + pin TSV codec.

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_json_roundtrip](/crates/oxide-library/src/primitive/symbol/tests/symbol_json_roundtrip.md) |
| related | [symbol_file_upsert_replaces_matching_uuid](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_upsert_replaces_matching_uuid.md) |
| related | [symbol_file_toml_round_trip_empty_symbol](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_toml_round_trip_empty_symbol.md) |
| related | [symbol_file_toml_round_trip_multi](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_toml_round_trip_multi.md) |
| related | [symbol_file_from_bytes_decodes_toml_envelope](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_from_bytes_decodes_toml_envelope.md) |
| related | [symbol_file_from_bytes_rejects_empty_payload](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_from_bytes_rejects_empty_payload.md) |
| related | [symbol_file_round_trip_with_full_pin_payload](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_round_trip_with_full_pin_payload.md) |
| related | [symbol_file_to_toml_emits_pins_as_literal_multiline](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_to_toml_emits_pins_as_literal_multiline.md) |
| related | [pins_to_tsv_empty_emits_header_only](/crates/oxide-library/src/primitive/symbol/tests/pins_to_tsv_empty_emits_header_only.md) |
| related | [pins_to_tsv_rejects_tab_in_cell](/crates/oxide-library/src/primitive/symbol/tests/pins_to_tsv_rejects_tab_in_cell.md) |
| related | [pins_to_tsv_rejects_newline_in_cell](/crates/oxide-library/src/primitive/symbol/tests/pins_to_tsv_rejects_newline_in_cell.md) |
| related | [pins_to_tsv_rejects_triple_quote_in_cell](/crates/oxide-library/src/primitive/symbol/tests/pins_to_tsv_rejects_triple_quote_in_cell.md) |
| related | [pins_from_tsv_rejects_schema_mismatch](/crates/oxide-library/src/primitive/symbol/tests/pins_from_tsv_rejects_schema_mismatch.md) |
| related | [pins_from_tsv_rejects_cell_count_mismatch](/crates/oxide-library/src/primitive/symbol/tests/pins_from_tsv_rejects_cell_count_mismatch.md) |
| related | [pin_direction_token_round_trip_all_variants](/crates/oxide-library/src/primitive/symbol/tests/pin_direction_token_round_trip_all_variants.md) |
| related | [pin_orientation_token_round_trip_all_variants](/crates/oxide-library/src/primitive/symbol/tests/pin_orientation_token_round_trip_all_variants.md) |
| related | [pin_symbol_kind_token_round_trip_all_variants](/crates/oxide-library/src/primitive/symbol/tests/pin_symbol_kind_token_round_trip_all_variants.md) |
| related | [symbol_file_unsupported_format_token_is_rejected](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_unsupported_format_token_is_rejected.md) |
| related | [pin_electrical_type_round_trip_all_variants](/crates/oxide-library/src/primitive/symbol/tests/pin_electrical_type_round_trip_all_variants.md) |
| related | [pin_orientation_round_trip_all_variants](/crates/oxide-library/src/primitive/symbol/tests/pin_orientation_round_trip_all_variants.md) |
| related | [symbol_graphic_kind_round_trip_each_variant](/crates/oxide-library/src/primitive/symbol/tests/symbol_graphic_kind_round_trip_each_variant.md) |
| related | [empty_symbol_starts_without_default_pins](/crates/oxide-library/src/primitive/symbol/tests/empty_symbol_starts_without_default_pins.md) |
| related | [part_count_round_trips](/crates/oxide-library/src/primitive/symbol/tests/part_count_round_trips.md) |
| related | [legacy_file_reconciles_part_count](/crates/oxide-library/src/primitive/symbol/tests/legacy_file_reconciles_part_count.md) |
| related | [graphic_part_number_round_trips](/crates/oxide-library/src/primitive/symbol/tests/graphic_part_number_round_trips.md) |
| related | [graphic_fill_round_trips](/crates/oxide-library/src/primitive/symbol/tests/graphic_fill_round_trips.md) |
| related | [graphic_missing_part_number_defaults_to_zero](/crates/oxide-library/src/primitive/symbol/tests/graphic_missing_part_number_defaults_to_zero.md) |
| related | [polygon_graphic_round_trips](/crates/oxide-library/src/primitive/symbol/tests/polygon_graphic_round_trips.md) |
| related | [symbol_file_with_no_polygon_graphic_stays_v1](/crates/oxide-library/src/primitive/symbol/tests/symbol_file_with_no_polygon_graphic_stays_v1.md) |
| related | [arc_symbol](/crates/oxide-library/src/primitive/symbol/tests/arc_symbol.md) |
| related | [arc_legacy_cw_signed_pair_migrates_to_ccw_swap_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_legacy_cw_signed_pair_migrates_to_ccw_swap_on_load.md) |
| related | [arc_wraparound_pair_already_in_range_is_unchanged_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_wraparound_pair_already_in_range_is_unchanged_on_load.md) |
| related | [arc_migration_is_a_load_time_fixed_point](/crates/oxide-library/src/primitive/symbol/tests/arc_migration_is_a_load_time_fixed_point.md) |
| related | [normalize_arc_endpoints_deg_swaps_a_cw_signed_pair](/crates/oxide-library/src/primitive/symbol/tests/normalize_arc_endpoints_deg_swaps_a_cw_signed_pair.md) |
| related | [normalize_arc_endpoints_deg_leaves_ccw_pairs_unswapped](/crates/oxide-library/src/primitive/symbol/tests/normalize_arc_endpoints_deg_leaves_ccw_pairs_unswapped.md) |
| related | [arc_full_turn_migrates_to_circle_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_full_turn_migrates_to_circle_on_load.md) |
| related | [arc_negative_full_turn_migrates_to_circle_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_negative_full_turn_migrates_to_circle_on_load.md) |
| related | [arc_near_full_turn_stays_an_arc_on_load](/crates/oxide-library/src/primitive/symbol/tests/arc_near_full_turn_stays_an_arc_on_load.md) |
