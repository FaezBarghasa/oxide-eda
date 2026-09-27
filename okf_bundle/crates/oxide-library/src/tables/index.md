# tables

## Classs

- [DenyNewFiles](DenyNewFiles.md) — RAII guard: while alive, `dir` rejects new-file creation, so
- [TableSchema](TableSchema.md) — Schema descriptor — held on the side for callers that need to reflect

## Functions

- [append_row](append_row.md) — Append a single row to the table file.
- [append_row_grows_the_file](append_row_grows_the_file.md) — `append_row` honours an empty file, then keeps growing it — and
- [datasheet_from_cell](datasheet_from_cell.md)
- [datasheet_to_cell](datasheet_to_cell.md)
- [delete_row](delete_row.md) — Remove the row whose `row_id` matches. Returns `NotFound` if no such row
- [delete_row_missing_returns_not_found](delete_row_missing_returns_not_found.md) — [test]
- [delete_row_removes_only_matching_id](delete_row_removes_only_matching_id.md) — `delete_row` removes the matching id, leaves others alone.
- [drop](drop.md)
- [drop](drop_1.md)
- [from_json_cell](from_json_cell.md)
- [has_stray_tmp](has_stray_tmp.md) — True if `dir` contains a leftover atomic-write temp sibling
- [hash_from_cell](hash_from_cell.md)
- [hash_from_cell_rejects_multibyte_cell](hash_from_cell_rejects_multibyte_cell.md) — A 64-BYTE cell whose bytes are not all ASCII hex is a recoverable
- [hash_round_trip_preserves_bytes](hash_round_trip_preserves_bytes.md) — Hash hex encode/decode is bit-exact.
- [hash_to_cell](hash_to_cell.md)
- [json_cell](json_cell.md) — ── (de)serialisation helpers ─────────────────────────────────────────────
- [lifecycle_from_cell](lifecycle_from_cell.md)
- [lifecycle_to_cell](lifecycle_to_cell.md)
- [mk_row](mk_row.md)
- [none_primitive_refs_encode_empty](none_primitive_refs_encode_empty.md) — `none` footprint and sim refs encode as empty strings; round-trip
- [on](on.md)
- [on](on_1.md)
- [opt_primitive_from_cell](opt_primitive_from_cell.md)
- [opt_primitive_to_cell](opt_primitive_to_cell.md)
- [read_missing_file_is_empty](read_missing_file_is_empty.md) — Empty file (no header) reads back as empty vec.
- [read_table](read_table.md) — Read every row from `path`. Empty file (header-only) yields an empty `Vec`.
- [record_to_row](record_to_row.md)
- [row_to_record](row_to_record.md)
- [schema_constant_matches_header_length](schema_constant_matches_header_length.md) — [test]
- [settle_deny](settle_deny.md) — Poll with a real probe write instead of trusting `icacls`'s exit
- [settle_deny](settle_deny_1.md) — Poll with a real probe write instead of trusting `icacls`'s exit
- [timestamp_from_cell](timestamp_from_cell.md)
- [timestamp_to_cell](timestamp_to_cell.md)
- [tsv_roundtrip_preserves_row](tsv_roundtrip_preserves_row.md) — Plan §6 step 1.3 — the foundational TSV round-trip.
- [update_row](update_row.md) — Replace the row whose `row_id` matches. Returns `NotFound` if no such
- [update_row_missing_returns_not_found](update_row_missing_returns_not_found.md) — [test]
- [update_row_replaces_in_place](update_row_replaces_in_place.md) — `update_row` replaces a row in-place by id.
- [write_table](write_table.md) — Replace the contents of `path` with `rows`. Creates parent directories
- [write_table_is_atomic_and_preserves_original_on_failure](write_table_is_atomic_and_preserves_original_on_failure.md) — `write_table` must go through `atomic_write`, not `fs::write`:
