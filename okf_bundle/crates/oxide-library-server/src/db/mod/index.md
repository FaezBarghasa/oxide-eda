# mod

## Classs

- [AppState](AppState.md) — Server-side state shared across all actix handlers.
- [ComponentRowRecord](ComponentRowRecord.md) — [derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
- [PrimitiveRecord](PrimitiveRecord.md) — [derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
- [PrimitiveSummary](PrimitiveSummary.md) — Summary record for a primitive (Symbol / Footprint / SimModel) — what the

## Functions

- [assert_primitive_table](assert_primitive_table.md) — ---------- Primitive query helpers ----------------------------------------
- [connect](connect.md) — Connect to a database backend. Supports memory and remote SurrealDB instances.
- [connect](connect_1.md) — Connect to a database backend. Supports memory and remote SurrealDB instances.
- [db](db.md)
- [db](db_1.md)
- [decode_err](decode_err.md)
- [delete_row](delete_row.md) — Delete a row. Returns `Ok(false)` if no matching row existed.
- [delete_row](delete_row_1.md) — Delete a row. Returns `Ok(false)` if no matching row existed.
- [fetch_footprint](fetch_footprint.md)
- [fetch_footprint](fetch_footprint_1.md)
- [fetch_primitive_payload](fetch_primitive_payload.md)
- [fetch_row](fetch_row.md)
- [fetch_row](fetch_row_1.md)
- [fetch_sim](fetch_sim.md)
- [fetch_sim](fetch_sim_1.md)
- [fetch_symbol](fetch_symbol.md)
- [fetch_symbol](fetch_symbol_1.md)
- [fixture_row](fixture_row.md)
- [insert_footprint](insert_footprint.md)
- [insert_footprint](insert_footprint_1.md)
- [insert_row](insert_row.md) — Insert a brand-new row. Returns `Ok(false)` when a row with the
- [insert_row](insert_row_1.md) — Insert a brand-new row. Returns `Ok(false)` when a row with the
- [insert_sim](insert_sim.md)
- [insert_sim](insert_sim_1.md)
- [insert_symbol](insert_symbol.md) — ── Primitive CRUD ────────────────────────────────────────────────────
- [insert_symbol](insert_symbol_1.md) — ── Primitive CRUD ────────────────────────────────────────────────────
- [list_footprints](list_footprints.md)
- [list_footprints](list_footprints_1.md)
- [list_primitive_summaries](list_primitive_summaries.md)
- [list_rows_in_table](list_rows_in_table.md) — Read every row in `table_name` for `library_id`, ordered by
- [list_rows_in_table](list_rows_in_table_1.md) — Read every row in `table_name` for `library_id`, ordered by
- [list_sims](list_sims.md)
- [list_sims](list_sims_1.md)
- [list_symbols](list_symbols.md)
- [list_symbols](list_symbols_1.md)
- [list_table_names](list_table_names.md) — List the names of every distinct table that has at least one row
- [list_table_names](list_table_names_1.md) — List the names of every distinct table that has at least one row
- [locks](locks.md)
- [locks](locks_1.md)
- [locks_arc](locks_arc.md) — Hand out a clone of the `Arc<LockManager>` for background tasks
- [locks_arc](locks_arc_1.md) — Hand out a clone of the `Arc<LockManager>` for background tasks
- [migrate](migrate.md) — Apply schema definitions to SurrealDB.
- [migrate](migrate_1.md) — Apply schema definitions to SurrealDB.
- [new_memory](new_memory.md) — Open an in-memory SurrealDB database.
- [new_memory](new_memory_1.md) — Open an in-memory SurrealDB database.
- [new_sqlite_memory](new_sqlite_memory.md) — Alias for backwards compatibility with tests and callers expecting `new_sqlite_memory`.
- [new_sqlite_memory](new_sqlite_memory_1.md) — Alias for backwards compatibility with tests and callers expecting `new_sqlite_memory`.
- [test_db_crud_direct](test_db_crud_direct.md) — [tokio::test]
- [update_row](update_row.md) — Update an existing row. Returns `Ok(false)` if no row with the
- [update_row](update_row_1.md) — Update an existing row. Returns `Ok(false)` if no row with the
- [upsert_primitive](upsert_primitive.md)
