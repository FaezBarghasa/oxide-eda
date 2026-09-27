---
okf_version: "0.2"
type: Module
title: tests
description: Unit tests for the .snxlib library-file codec.
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests
language: rust
---

# tests

Unit tests for the .snxlib library-file codec.

## Docstring

Unit tests for the .snxlib library-file codec.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture_manifest](/crates/oxide-library/src/library_file/tests/fixture_manifest.md) |
| related | [make_row](/crates/oxide-library/src/library_file/tests/make_row.md) |
| related | [fixture_table_resistors](/crates/oxide-library/src/library_file/tests/fixture_table_resistors.md) |
| related | [round_trip_no_tables](/crates/oxide-library/src/library_file/tests/round_trip_no_tables.md) |
| related | [round_trip_single_table](/crates/oxide-library/src/library_file/tests/round_trip_single_table.md) |
| related | [round_trip_multiple_tables_user_defined_columns](/crates/oxide-library/src/library_file/tests/round_trip_multiple_tables_user_defined_columns.md) |
| related | [write_is_byte_idempotent_under_round_trip](/crates/oxide-library/src/library_file/tests/write_is_byte_idempotent_under_round_trip.md) |
| related | [parse_rejects_unsupported_format_token](/crates/oxide-library/src/library_file/tests/parse_rejects_unsupported_format_token.md) |
| related | [parse_rejects_duplicate_columns](/crates/oxide-library/src/library_file/tests/parse_rejects_duplicate_columns.md) |
| related | [parse_rejects_cell_count_mismatch](/crates/oxide-library/src/library_file/tests/parse_rejects_cell_count_mismatch.md) |
| related | [header_only_tsv_is_empty_rows](/crates/oxide-library/src/library_file/tests/header_only_tsv_is_empty_rows.md) |
| related | [parse_rejects_empty_tsv](/crates/oxide-library/src/library_file/tests/parse_rejects_empty_tsv.md) |
| related | [write_rejects_cell_with_tab](/crates/oxide-library/src/library_file/tests/write_rejects_cell_with_tab.md) |
| related | [write_rejects_cell_with_newline](/crates/oxide-library/src/library_file/tests/write_rejects_cell_with_newline.md) |
| related | [write_rejects_cell_with_triple_single_quote](/crates/oxide-library/src/library_file/tests/write_rejects_cell_with_triple_single_quote.md) |
| related | [write_rejects_column_name_with_tab](/crates/oxide-library/src/library_file/tests/write_rejects_column_name_with_tab.md) |
| related | [parse_example_from_plan](/crates/oxide-library/src/library_file/tests/parse_example_from_plan.md) |
| related | [cell_accessor_is_schema_scoped](/crates/oxide-library/src/library_file/tests/cell_accessor_is_schema_scoped.md) |
| related | [empty_cells_round_trip](/crates/oxide-library/src/library_file/tests/empty_cells_round_trip.md) |
| related | [write_emits_canonical_row_order_by_row_id](/crates/oxide-library/src/library_file/tests/write_emits_canonical_row_order_by_row_id.md) |
| related | [write_preserves_insertion_order_when_no_row_id](/crates/oxide-library/src/library_file/tests/write_preserves_insertion_order_when_no_row_id.md) |
| related | [column_types_round_trip](/crates/oxide-library/src/library_file/tests/column_types_round_trip.md) |
| related | [untyped_tables_skip_column_types_block](/crates/oxide-library/src/library_file/tests/untyped_tables_skip_column_types_block.md) |
| related | [parse_rejects_column_type_for_unknown_column](/crates/oxide-library/src/library_file/tests/parse_rejects_column_type_for_unknown_column.md) |
| related | [parse_rejects_unknown_type_token](/crates/oxide-library/src/library_file/tests/parse_rejects_unknown_type_token.md) |
| related | [column_type_parse_rejects_empty_enum](/crates/oxide-library/src/library_file/tests/column_type_parse_rejects_empty_enum.md) |
| related | [column_type_token_round_trip_all_variants](/crates/oxide-library/src/library_file/tests/column_type_token_round_trip_all_variants.md) |
| related | [column_order_preserved](/crates/oxide-library/src/library_file/tests/column_order_preserved.md) |
