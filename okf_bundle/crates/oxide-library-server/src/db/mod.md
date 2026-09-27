---
okf_version: "0.2"
type: Module
title: db
description: "Database layer — SurrealDB management, schema definitions, and component-row /"
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod
language: rust
---

# db

Database layer — SurrealDB management, schema definitions, and component-row /

## Docstring

Database layer — SurrealDB management, schema definitions, and component-row /
primitive persistence helpers used by the route handlers.

Components live as rows inside category tables (Altium DBLib
model). This module exposes:

* primitive CRUD (`insert_symbol` / `fetch_symbol` / …) —
primitives stay file-shaped under the row model;
* row CRUD (`insert_row` / `fetch_row` / `update_row` / `delete_row`) +
table-name listing (`list_table_names` / `list_rows_in_table`) —
backing the `/tables` and `/rows` HTTP routes.

Powered by SurrealDB (embedded in-memory and local/remote engine).

## Relationships

| Type | Target |
|------|--------|
| related | [PrimitiveSummary](/crates/oxide-library-server/src/db/mod/PrimitiveSummary.md) |
| related | [ComponentRowRecord](/crates/oxide-library-server/src/db/mod/ComponentRowRecord.md) |
| related | [PrimitiveRecord](/crates/oxide-library-server/src/db/mod/PrimitiveRecord.md) |
| related | [AppState](/crates/oxide-library-server/src/db/mod/AppState.md) |
| related | [new_memory](/crates/oxide-library-server/src/db/mod/new_memory.md) |
| related | [new_sqlite_memory](/crates/oxide-library-server/src/db/mod/new_sqlite_memory.md) |
| related | [connect](/crates/oxide-library-server/src/db/mod/connect.md) |
| related | [db](/crates/oxide-library-server/src/db/mod/db.md) |
| related | [locks](/crates/oxide-library-server/src/db/mod/locks.md) |
| related | [locks_arc](/crates/oxide-library-server/src/db/mod/locks_arc.md) |
| related | [migrate](/crates/oxide-library-server/src/db/mod/migrate.md) |
| related | [insert_row](/crates/oxide-library-server/src/db/mod/insert_row.md) |
| related | [update_row](/crates/oxide-library-server/src/db/mod/update_row.md) |
| related | [fetch_row](/crates/oxide-library-server/src/db/mod/fetch_row.md) |
| related | [delete_row](/crates/oxide-library-server/src/db/mod/delete_row.md) |
| related | [list_table_names](/crates/oxide-library-server/src/db/mod/list_table_names.md) |
| related | [list_rows_in_table](/crates/oxide-library-server/src/db/mod/list_rows_in_table.md) |
| related | [insert_symbol](/crates/oxide-library-server/src/db/mod/insert_symbol.md) |
| related | [fetch_symbol](/crates/oxide-library-server/src/db/mod/fetch_symbol.md) |
| related | [list_symbols](/crates/oxide-library-server/src/db/mod/list_symbols.md) |
| related | [insert_footprint](/crates/oxide-library-server/src/db/mod/insert_footprint.md) |
| related | [fetch_footprint](/crates/oxide-library-server/src/db/mod/fetch_footprint.md) |
| related | [list_footprints](/crates/oxide-library-server/src/db/mod/list_footprints.md) |
| related | [insert_sim](/crates/oxide-library-server/src/db/mod/insert_sim.md) |
| related | [fetch_sim](/crates/oxide-library-server/src/db/mod/fetch_sim.md) |
| related | [list_sims](/crates/oxide-library-server/src/db/mod/list_sims.md) |
| related | [new_memory](/crates/oxide-library-server/src/db/mod/new_memory.md) |
| related | [new_sqlite_memory](/crates/oxide-library-server/src/db/mod/new_sqlite_memory.md) |
| related | [connect](/crates/oxide-library-server/src/db/mod/connect.md) |
| related | [db](/crates/oxide-library-server/src/db/mod/db.md) |
| related | [locks](/crates/oxide-library-server/src/db/mod/locks.md) |
| related | [locks_arc](/crates/oxide-library-server/src/db/mod/locks_arc.md) |
| related | [migrate](/crates/oxide-library-server/src/db/mod/migrate.md) |
| related | [insert_row](/crates/oxide-library-server/src/db/mod/insert_row.md) |
| related | [update_row](/crates/oxide-library-server/src/db/mod/update_row.md) |
| related | [fetch_row](/crates/oxide-library-server/src/db/mod/fetch_row.md) |
| related | [delete_row](/crates/oxide-library-server/src/db/mod/delete_row.md) |
| related | [list_table_names](/crates/oxide-library-server/src/db/mod/list_table_names.md) |
| related | [list_rows_in_table](/crates/oxide-library-server/src/db/mod/list_rows_in_table.md) |
| related | [insert_symbol](/crates/oxide-library-server/src/db/mod/insert_symbol.md) |
| related | [fetch_symbol](/crates/oxide-library-server/src/db/mod/fetch_symbol.md) |
| related | [list_symbols](/crates/oxide-library-server/src/db/mod/list_symbols.md) |
| related | [insert_footprint](/crates/oxide-library-server/src/db/mod/insert_footprint.md) |
| related | [fetch_footprint](/crates/oxide-library-server/src/db/mod/fetch_footprint.md) |
| related | [list_footprints](/crates/oxide-library-server/src/db/mod/list_footprints.md) |
| related | [insert_sim](/crates/oxide-library-server/src/db/mod/insert_sim.md) |
| related | [fetch_sim](/crates/oxide-library-server/src/db/mod/fetch_sim.md) |
| related | [list_sims](/crates/oxide-library-server/src/db/mod/list_sims.md) |
| related | [assert_primitive_table](/crates/oxide-library-server/src/db/mod/assert_primitive_table.md) |
| related | [upsert_primitive](/crates/oxide-library-server/src/db/mod/upsert_primitive.md) |
| related | [fetch_primitive_payload](/crates/oxide-library-server/src/db/mod/fetch_primitive_payload.md) |
| related | [list_primitive_summaries](/crates/oxide-library-server/src/db/mod/list_primitive_summaries.md) |
| related | [decode_err](/crates/oxide-library-server/src/db/mod/decode_err.md) |
| related | [fixture_row](/crates/oxide-library-server/src/db/mod/fixture_row.md) |
| related | [test_db_crud_direct](/crates/oxide-library-server/src/db/mod/test_db_crud_direct.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [surrealdb](/_dependencies/cargo/surrealdb.md) |
