---
okf_version: "0.2"
type: Function
title: fixture_manifest
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/fixture_manifest
language: rust
---

# fixture_manifest

## Signature

```rust
fn fixture_manifest() -> SnxlibManifest
```

## Source
Lines 4–17 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
| called_by | [column_order_preserved](/crates/oxide-library/src/library_file/tests/column_order_preserved.md) |
| called_by | [column_types_round_trip](/crates/oxide-library/src/library_file/tests/column_types_round_trip.md) |
| called_by | [empty_cells_round_trip](/crates/oxide-library/src/library_file/tests/empty_cells_round_trip.md) |
| called_by | [round_trip_multiple_tables_user_defined_columns](/crates/oxide-library/src/library_file/tests/round_trip_multiple_tables_user_defined_columns.md) |
| called_by | [round_trip_no_tables](/crates/oxide-library/src/library_file/tests/round_trip_no_tables.md) |
| called_by | [round_trip_single_table](/crates/oxide-library/src/library_file/tests/round_trip_single_table.md) |
| called_by | [untyped_tables_skip_column_types_block](/crates/oxide-library/src/library_file/tests/untyped_tables_skip_column_types_block.md) |
| called_by | [write_emits_canonical_row_order_by_row_id](/crates/oxide-library/src/library_file/tests/write_emits_canonical_row_order_by_row_id.md) |
| called_by | [write_is_byte_idempotent_under_round_trip](/crates/oxide-library/src/library_file/tests/write_is_byte_idempotent_under_round_trip.md) |
| called_by | [write_preserves_insertion_order_when_no_row_id](/crates/oxide-library/src/library_file/tests/write_preserves_insertion_order_when_no_row_id.md) |
| called_by | [write_rejects_cell_with_newline](/crates/oxide-library/src/library_file/tests/write_rejects_cell_with_newline.md) |
| called_by | [write_rejects_cell_with_tab](/crates/oxide-library/src/library_file/tests/write_rejects_cell_with_tab.md) |
| called_by | [write_rejects_cell_with_triple_single_quote](/crates/oxide-library/src/library_file/tests/write_rejects_cell_with_triple_single_quote.md) |
| called_by | [write_rejects_column_name_with_tab](/crates/oxide-library/src/library_file/tests/write_rejects_column_name_with_tab.md) |
