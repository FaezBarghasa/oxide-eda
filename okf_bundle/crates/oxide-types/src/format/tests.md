---
okf_version: "0.2"
type: Module
title: tests
description: "Round-trip / serialization tests for the `.snxsch` / `.snxpcb`"
resource: crates/oxide-types/src/format/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:16:55Z"
concept_id: crates/oxide-types/src/format/tests
language: rust
---

# tests

Round-trip / serialization tests for the `.snxsch` / `.snxpcb`

## Docstring

Round-trip / serialization tests for the `.snxsch` / `.snxpcb`
wire format, including the issue-#96 save-corruption regressions.
Pure code motion out of `mod.rs`; the assertions are unchanged —
this suite is the data-loss guard for the format layer.

## Relationships

| Type | Target |
|------|--------|
| related | [assert_btree_map](/crates/oxide-types/src/format/tests/assert_btree_map.md) |
| related | [empty_sheet](/crates/oxide-types/src/format/tests/empty_sheet.md) |
| related | [empty_board](/crates/oxide-types/src/format/tests/empty_board.md) |
| related | [snxsch_round_trip_empty](/crates/oxide-types/src/format/tests/snxsch_round_trip_empty.md) |
| related | [encode_decode_cell_round_trips_dangerous_characters](/crates/oxide-types/src/format/tests/encode_decode_cell_round_trips_dangerous_characters.md) |
| related | [escape_tsv_body_for_toml_never_leaves_a_triple_quote_run](/crates/oxide-types/src/format/tests/escape_tsv_body_for_toml_never_leaves_a_triple_quote_run.md) |
| related | [label_with_text](/crates/oxide-types/src/format/tests/label_with_text.md) |
| related | [snxsch_survives_backslash_and_quote_in_label_text](/crates/oxide-types/src/format/tests/snxsch_survives_backslash_and_quote_in_label_text.md) |
| related | [snxsch_survives_c0_control_bytes_in_label_text](/crates/oxide-types/src/format/tests/snxsch_survives_c0_control_bytes_in_label_text.md) |
| related | [snxsch_does_not_drop_a_literal_dash_label](/crates/oxide-types/src/format/tests/snxsch_does_not_drop_a_literal_dash_label.md) |
| related | [snxsch_survives_embedded_newline_in_label_text](/crates/oxide-types/src/format/tests/snxsch_survives_embedded_newline_in_label_text.md) |
| related | [snxpcb_round_trip_empty](/crates/oxide-types/src/format/tests/snxpcb_round_trip_empty.md) |
| related | [rejects_wrong_format_version](/crates/oxide-types/src/format/tests/rejects_wrong_format_version.md) |
| related | [rejects_wrong_pcb_format_version](/crates/oxide-types/src/format/tests/rejects_wrong_pcb_format_version.md) |
| related | [snxsch_includes_tsv_blocks_substring](/crates/oxide-types/src/format/tests/snxsch_includes_tsv_blocks_substring.md) |
| related | [snxpcb_includes_tsv_blocks_substring](/crates/oxide-types/src/format/tests/snxpcb_includes_tsv_blocks_substring.md) |
| related | [sample_symbol](/crates/oxide-types/src/format/tests/sample_symbol.md) |
| related | [sample_wire](/crates/oxide-types/src/format/tests/sample_wire.md) |
| related | [show_serialised_pcb_for_inspection](/crates/oxide-types/src/format/tests/show_serialised_pcb_for_inspection.md) |
| related | [snxsch_round_trip_with_data](/crates/oxide-types/src/format/tests/snxsch_round_trip_with_data.md) |
| related | [snxsch_without_junction_extras_defaults_to_user_placed](/crates/oxide-types/src/format/tests/snxsch_without_junction_extras_defaults_to_user_placed.md) |
| related | [snxpcb_round_trip_with_data](/crates/oxide-types/src/format/tests/snxpcb_round_trip_with_data.md) |
| related | [tsv_writer_pads_columns_for_legibility](/crates/oxide-types/src/format/tests/tsv_writer_pads_columns_for_legibility.md) |
| related | [tsv_parser_rejects_header_mismatch](/crates/oxide-types/src/format/tests/tsv_parser_rejects_header_mismatch.md) |
| related | [tsv_parser_rejects_cell_count_mismatch](/crates/oxide-types/src/format/tests/tsv_parser_rejects_cell_count_mismatch.md) |
| related | [integer_nanometre_coords_survive_round_trip](/crates/oxide-types/src/format/tests/integer_nanometre_coords_survive_round_trip.md) |
| related | [extras_preserve_symbol_fields](/crates/oxide-types/src/format/tests/extras_preserve_symbol_fields.md) |
| related | [schematic_extras_are_byte_identical_after_parse](/crates/oxide-types/src/format/tests/schematic_extras_are_byte_identical_after_parse.md) |
