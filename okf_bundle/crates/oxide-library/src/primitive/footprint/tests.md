---
okf_version: "0.2"
type: Module
title: tests
description: Unit tests for the footprint primitive + pad TSV codec.
resource: crates/oxide-library/src/primitive/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/tests
language: rust
---

# tests

Unit tests for the footprint primitive + pad TSV codec.

## Docstring

Unit tests for the footprint primitive + pad TSV codec.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture_pad](/crates/oxide-library/src/primitive/footprint/tests/fixture_pad.md) |
| related | [footprint_json_roundtrip_with_body3d](/crates/oxide-library/src/primitive/footprint/tests/footprint_json_roundtrip_with_body3d.md) |
| related | [body3d_default_is_grey_extrude_at_zero_offset](/crates/oxide-library/src/primitive/footprint/tests/body3d_default_is_grey_extrude_at_zero_offset.md) |
| related | [pad_kind_round_trip_all_variants](/crates/oxide-library/src/primitive/footprint/tests/pad_kind_round_trip_all_variants.md) |
| related | [pad_shape_round_trip_each_variant](/crates/oxide-library/src/primitive/footprint/tests/pad_shape_round_trip_each_variant.md) |
| related | [body_shape_round_trip_all_variants](/crates/oxide-library/src/primitive/footprint/tests/body_shape_round_trip_all_variants.md) |
| related | [empty_footprint_has_no_pads](/crates/oxide-library/src/primitive/footprint/tests/empty_footprint_has_no_pads.md) |
| related | [step_attachment_round_trip](/crates/oxide-library/src/primitive/footprint/tests/step_attachment_round_trip.md) |
| related | [footprint_file_toml_round_trip_empty](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_toml_round_trip_empty.md) |
| related | [footprint_file_toml_round_trip_with_pads](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_toml_round_trip_with_pads.md) |
| related | [footprint_file_toml_round_trip_multi](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_toml_round_trip_multi.md) |
| related | [footprint_file_from_bytes_decodes_toml_envelope](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_from_bytes_decodes_toml_envelope.md) |
| related | [footprint_file_from_bytes_rejects_empty_payload](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_from_bytes_rejects_empty_payload.md) |
| related | [pad_kind_token_round_trip_all_variants](/crates/oxide-library/src/primitive/footprint/tests/pad_kind_token_round_trip_all_variants.md) |
| related | [pad_shape_token_round_trip_each_variant](/crates/oxide-library/src/primitive/footprint/tests/pad_shape_token_round_trip_each_variant.md) |
| related | [pads_to_tsv_empty_emits_header_only](/crates/oxide-library/src/primitive/footprint/tests/pads_to_tsv_empty_emits_header_only.md) |
| related | [pads_to_tsv_rejects_tab_in_cell](/crates/oxide-library/src/primitive/footprint/tests/pads_to_tsv_rejects_tab_in_cell.md) |
| related | [pads_from_tsv_rejects_schema_mismatch](/crates/oxide-library/src/primitive/footprint/tests/pads_from_tsv_rejects_schema_mismatch.md) |
| related | [pads_from_tsv_rejects_drill_slot_without_diameter](/crates/oxide-library/src/primitive/footprint/tests/pads_from_tsv_rejects_drill_slot_without_diameter.md) |
| related | [footprint_file_round_trip_with_full_pad_payload](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_round_trip_with_full_pad_payload.md) |
| related | [footprint_file_round_trip_with_custom_polygon_pad](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_round_trip_with_custom_polygon_pad.md) |
| related | [footprint_file_to_toml_emits_pads_as_literal_multiline](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_to_toml_emits_pads_as_literal_multiline.md) |
| related | [footprint_file_unsupported_format_token_is_rejected](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_unsupported_format_token_is_rejected.md) |
| related | [footprint_file_get_by_uuid](/crates/oxide-library/src/primitive/footprint/tests/footprint_file_get_by_uuid.md) |
