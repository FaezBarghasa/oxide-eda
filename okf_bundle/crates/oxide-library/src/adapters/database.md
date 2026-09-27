---
okf_version: "0.2"
type: Module
title: database
description: "`LibraryAdapter` over the HTTP API exposed by `oxide-library-server`."
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database
language: rust
---

# database

`LibraryAdapter` over the HTTP API exposed by `oxide-library-server`.

## Docstring

`LibraryAdapter` over the HTTP API exposed by `oxide-library-server`.

Synchronous facade for the trait. Row CRUD speaks to the
`/tables` / `/rows` routes; primitive (`/symbols` / `/footprints`
/ `/sims`) wiring is unchanged because primitives stay
file-shaped under the DBLib model.

Routes are addressed by a `library_id` query parameter — the
adapter sources its own from `manifest().library.library_id`.
Mutating calls carry their commit message in the
`x-oxide-message` header so the server-side audit log has it
(the DB backend doesn't have its own commit graph the way
`LocalGitAdapter` does — see TODO around the `audit_log` table
below).

## Relationships

| Type | Target |
|------|--------|
| related | [DatabaseAdapter](/crates/oxide-library/src/adapters/database/DatabaseAdapter.md) |
| related | [new](/crates/oxide-library/src/adapters/database/new.md) |
| related | [from_snxlib](/crates/oxide-library/src/adapters/database/from_snxlib.md) |
| related | [with_token](/crates/oxide-library/src/adapters/database/with_token.md) |
| related | [base_url](/crates/oxide-library/src/adapters/database/base_url.md) |
| related | [holder](/crates/oxide-library/src/adapters/database/holder.md) |
| related | [url](/crates/oxide-library/src/adapters/database/url.md) |
| related | [encode_segment](/crates/oxide-library/src/adapters/database/encode_segment.md) |
| related | [auth](/crates/oxide-library/src/adapters/database/auth.md) |
| related | [get_primitive_json](/crates/oxide-library/src/adapters/database/get_primitive_json.md) |
| related | [post_primitive_json](/crates/oxide-library/src/adapters/database/post_primitive_json.md) |
| related | [list_primitives_json](/crates/oxide-library/src/adapters/database/list_primitives_json.md) |
| related | [library_id_query](/crates/oxide-library/src/adapters/database/library_id_query.md) |
| related | [new](/crates/oxide-library/src/adapters/database/new.md) |
| related | [from_snxlib](/crates/oxide-library/src/adapters/database/from_snxlib.md) |
| related | [with_token](/crates/oxide-library/src/adapters/database/with_token.md) |
| related | [base_url](/crates/oxide-library/src/adapters/database/base_url.md) |
| related | [holder](/crates/oxide-library/src/adapters/database/holder.md) |
| related | [url](/crates/oxide-library/src/adapters/database/url.md) |
| related | [encode_segment](/crates/oxide-library/src/adapters/database/encode_segment.md) |
| related | [auth](/crates/oxide-library/src/adapters/database/auth.md) |
| related | [get_primitive_json](/crates/oxide-library/src/adapters/database/get_primitive_json.md) |
| related | [post_primitive_json](/crates/oxide-library/src/adapters/database/post_primitive_json.md) |
| related | [list_primitives_json](/crates/oxide-library/src/adapters/database/list_primitives_json.md) |
| related | [library_id_query](/crates/oxide-library/src/adapters/database/library_id_query.md) |
| related | [manifest](/crates/oxide-library/src/adapters/database/manifest.md) |
| related | [list_tables](/crates/oxide-library/src/adapters/database/list_tables.md) |
| related | [read_table](/crates/oxide-library/src/adapters/database/read_table.md) |
| related | [iter_rows](/crates/oxide-library/src/adapters/database/iter_rows.md) |
| related | [read_row](/crates/oxide-library/src/adapters/database/read_row.md) |
| related | [read_row_by_pn](/crates/oxide-library/src/adapters/database/read_row_by_pn.md) |
| related | [insert_row](/crates/oxide-library/src/adapters/database/insert_row.md) |
| related | [update_row](/crates/oxide-library/src/adapters/database/update_row.md) |
| related | [delete_row](/crates/oxide-library/src/adapters/database/delete_row.md) |
| related | [get_symbol](/crates/oxide-library/src/adapters/database/get_symbol.md) |
| related | [get_footprint](/crates/oxide-library/src/adapters/database/get_footprint.md) |
| related | [get_sim](/crates/oxide-library/src/adapters/database/get_sim.md) |
| related | [save_symbol](/crates/oxide-library/src/adapters/database/save_symbol.md) |
| related | [save_footprint](/crates/oxide-library/src/adapters/database/save_footprint.md) |
| related | [save_sim](/crates/oxide-library/src/adapters/database/save_sim.md) |
| related | [list_symbols](/crates/oxide-library/src/adapters/database/list_symbols.md) |
| related | [list_footprints](/crates/oxide-library/src/adapters/database/list_footprints.md) |
| related | [list_sims](/crates/oxide-library/src/adapters/database/list_sims.md) |
| related | [manifest](/crates/oxide-library/src/adapters/database/manifest.md) |
| related | [list_tables](/crates/oxide-library/src/adapters/database/list_tables.md) |
| related | [read_table](/crates/oxide-library/src/adapters/database/read_table.md) |
| related | [iter_rows](/crates/oxide-library/src/adapters/database/iter_rows.md) |
| related | [read_row](/crates/oxide-library/src/adapters/database/read_row.md) |
| related | [read_row_by_pn](/crates/oxide-library/src/adapters/database/read_row_by_pn.md) |
| related | [insert_row](/crates/oxide-library/src/adapters/database/insert_row.md) |
| related | [update_row](/crates/oxide-library/src/adapters/database/update_row.md) |
| related | [delete_row](/crates/oxide-library/src/adapters/database/delete_row.md) |
| related | [get_symbol](/crates/oxide-library/src/adapters/database/get_symbol.md) |
| related | [get_footprint](/crates/oxide-library/src/adapters/database/get_footprint.md) |
| related | [get_sim](/crates/oxide-library/src/adapters/database/get_sim.md) |
| related | [save_symbol](/crates/oxide-library/src/adapters/database/save_symbol.md) |
| related | [save_footprint](/crates/oxide-library/src/adapters/database/save_footprint.md) |
| related | [save_sim](/crates/oxide-library/src/adapters/database/save_sim.md) |
| related | [list_symbols](/crates/oxide-library/src/adapters/database/list_symbols.md) |
| related | [list_footprints](/crates/oxide-library/src/adapters/database/list_footprints.md) |
| related | [list_sims](/crates/oxide-library/src/adapters/database/list_sims.md) |
| related | [synthesize_legacy_manifest](/crates/oxide-library/src/adapters/database/synthesize_legacy_manifest.md) |
| related | [from_snxlib_round_trips_database_mode](/crates/oxide-library/src/adapters/database/from_snxlib_round_trips_database_mode.md) |
| related | [from_snxlib_rejects_non_database_mode](/crates/oxide-library/src/adapters/database/from_snxlib_rejects_non_database_mode.md) |
| related | [with_token_round_trips_holder_and_url](/crates/oxide-library/src/adapters/database/with_token_round_trips_holder_and_url.md) |
| related | [encode_segment_passes_through_unreserved](/crates/oxide-library/src/adapters/database/encode_segment_passes_through_unreserved.md) |
| related | [encode_segment_escapes_path_breakers](/crates/oxide-library/src/adapters/database/encode_segment_escapes_path_breakers.md) |
| related | [encode_segment_handles_utf8](/crates/oxide-library/src/adapters/database/encode_segment_handles_utf8.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
