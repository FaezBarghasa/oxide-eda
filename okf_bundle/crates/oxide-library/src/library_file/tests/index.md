# tests

## Functions

- [cell_accessor_is_schema_scoped](cell_accessor_is_schema_scoped.md) — `LibraryTable::cell` returns `None` for out-of-schema columns even
- [column_order_preserved](column_order_preserved.md) — Column order from the header is preserved through round-trip.
- [column_type_parse_rejects_empty_enum](column_type_parse_rejects_empty_enum.md) — `ColumnType::Enum` requires at least one non-empty value —
- [column_type_token_round_trip_all_variants](column_type_token_round_trip_all_variants.md) — Token-level round-trip for every variant — paranoia test that
- [column_types_round_trip](column_types_round_trip.md) — `column_types` map round-trips through the
- [empty_cells_round_trip](empty_cells_round_trip.md) — Empty cells preserve through round-trip — common in optional
- [fixture_manifest](fixture_manifest.md)
- [fixture_table_resistors](fixture_table_resistors.md)
- [header_only_tsv_is_empty_rows](header_only_tsv_is_empty_rows.md) — Header-only TSV (no body rows) parses to an empty `rows` vec.
- [make_row](make_row.md)
- [parse_example_from_plan](parse_example_from_plan.md) — A direct port of the §1 plan example. Anchors the format against
- [parse_rejects_cell_count_mismatch](parse_rejects_cell_count_mismatch.md) — [test]
- [parse_rejects_column_type_for_unknown_column](parse_rejects_column_type_for_unknown_column.md) — `column_types` with a key that doesn't exist in the TSV header
- [parse_rejects_duplicate_columns](parse_rejects_duplicate_columns.md) — [test]
- [parse_rejects_empty_tsv](parse_rejects_empty_tsv.md) — Empty TSV (no header at all) is rejected — there's no schema to
- [parse_rejects_unknown_type_token](parse_rejects_unknown_type_token.md) — Unknown type tokens fail loudly rather than silently treating
- [parse_rejects_unsupported_format_token](parse_rejects_unsupported_format_token.md) — Format token mismatches are loud (rather than silently parsing as
- [round_trip_multiple_tables_user_defined_columns](round_trip_multiple_tables_user_defined_columns.md) — Two tables with *different* column schemas — proves columns are
- [round_trip_no_tables](round_trip_no_tables.md) — The foundational round-trip — a library with no tables still
- [round_trip_single_table](round_trip_single_table.md) — Single-table round-trip with the columns + rows from the plan
- [untyped_tables_skip_column_types_block](untyped_tables_skip_column_types_block.md) — Untyped tables round-trip without emitting an empty
- [write_emits_canonical_row_order_by_row_id](write_emits_canonical_row_order_by_row_id.md) — Canonical row order on write — rows must emit sorted by `row_id`
- [write_is_byte_idempotent_under_round_trip](write_is_byte_idempotent_under_round_trip.md) — Idempotence — two consecutive writes produce byte-equal output.
- [write_preserves_insertion_order_when_no_row_id](write_preserves_insertion_order_when_no_row_id.md) — Tables without a `row_id` column preserve insertion order — no
- [write_rejects_cell_with_newline](write_rejects_cell_with_newline.md) — [test]
- [write_rejects_cell_with_tab](write_rejects_cell_with_tab.md) — [test]
- [write_rejects_cell_with_triple_single_quote](write_rejects_cell_with_triple_single_quote.md) — [test]
- [write_rejects_column_name_with_tab](write_rejects_column_name_with_tab.md) — [test]
