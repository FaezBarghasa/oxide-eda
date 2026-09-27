# where_used

## Classs

- [Entry](Entry.md) — [derive(Clone, Debug, PartialEq, Eq)]
- [UseSite](UseSite.md) — One occurrence of a row on a schematic sheet.
- [WhereUsedIndex](WhereUsedIndex.md) — Reverse index from row id → list of [`UseSite`].

## Functions

- [_assert_send_not_sync](assert_send_not_sync.md)
- [add_primitive_links](add_primitive_links.md)
- [add_primitive_links](add_primitive_links_1.md)
- [drop_project](drop_project.md) — Drop every entry for `project` (called on project close).
- [drop_project](drop_project_1.md) — Drop every entry for `project` (called on project close).
- [fixture_row](fixture_row.md)
- [ingest_row](ingest_row.md) — Replace the `primitive_to_rows` entries for a single row (idempotent).
- [ingest_row](ingest_row_1.md) — Replace the `primitive_to_rows` entries for a single row (idempotent).
- [ingest_row_replaces_prior_primitive_links](ingest_row_replaces_prior_primitive_links.md) — [test]
- [ingest_sheet](ingest_sheet.md) — Replace all entries for `sheet` under `project` with `refs`.
- [ingest_sheet](ingest_sheet_1.md) — Replace all entries for `sheet` under `project` with `refs`.
- [ingesting_empty_refs_clears_a_previous_sheet_entry](ingesting_empty_refs_clears_a_previous_sheet_entry.md) — [test]
- [is_send](is_send.md)
- [new](new.md) — Construct an empty index.
- [new](new_1.md) — Construct an empty index.
- [new_index_is_empty](new_index_is_empty.md) — [test]
- [rebuild_from_rows](rebuild_from_rows.md) — Rebuild the `primitive_to_rows` reverse index from a row scan
- [rebuild_from_rows](rebuild_from_rows_1.md) — Rebuild the `primitive_to_rows` reverse index from a row scan
- [rebuild_from_rows_replaces_state](rebuild_from_rows_replaces_state.md) — [test]
- [rows_for_primitive](rows_for_primitive.md) — All rows that reference the given primitive. Returned slice is empty
- [rows_for_primitive](rows_for_primitive_1.md) — All rows that reference the given primitive. Returned slice is empty
- [rows_for_primitive_returns_referencing_row](rows_for_primitive_returns_referencing_row.md) — [test]
- [where_used](where_used.md) — Find every site where `row_id` is used. Returned order is
- [where_used](where_used_1.md) — Find every site where `row_id` is used. Returned order is
