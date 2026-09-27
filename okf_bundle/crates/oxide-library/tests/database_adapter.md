---
okf_version: "0.2"
type: Module
title: database_adapter
description: "Wiremock-backed integration tests for `DatabaseAdapter`."
resource: crates/oxide-library/tests/database_adapter.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/database_adapter
language: rust
---

# database_adapter

Wiremock-backed integration tests for `DatabaseAdapter`.

## Docstring

Wiremock-backed integration tests for `DatabaseAdapter`.

Row CRUD speaks to the `/tables` and `/rows` routes; primitive
(`/symbols` / `/footprints` / `/sims`) coverage stays unchanged
under the DBLib model.

Mirrors the `distributor_*` test layout — a private tokio runtime
drives a `MockServer`; the adapter (which uses `reqwest::blocking`)
runs on a standard thread so its blocking calls don't deadlock the
runtime.

## Relationships

| Type | Target |
|------|--------|
| related | [with_mock_server](/crates/oxide-library/tests/database_adapter/with_mock_server.md) |
| related | [fixture_symbol](/crates/oxide-library/tests/database_adapter/fixture_symbol.md) |
| related | [get_symbol_round_trips_through_get_symbols_uuid](/crates/oxide-library/tests/database_adapter/get_symbol_round_trips_through_get_symbols_uuid.md) |
| related | [save_symbol_posts_to_symbols_with_message_header](/crates/oxide-library/tests/database_adapter/save_symbol_posts_to_symbols_with_message_header.md) |
| related | [save_sim_posts_to_sims_route](/crates/oxide-library/tests/database_adapter/save_sim_posts_to_sims_route.md) |
| related | [list_symbols_round_trips_through_get_symbols](/crates/oxide-library/tests/database_adapter/list_symbols_round_trips_through_get_symbols.md) |
| related | [get_symbol_404_maps_to_not_found](/crates/oxide-library/tests/database_adapter/get_symbol_404_maps_to_not_found.md) |
| related | [mk_row](/crates/oxide-library/tests/database_adapter/mk_row.md) |
| related | [database_round_trip_row](/crates/oxide-library/tests/database_adapter/database_round_trip_row.md) |
| related | [database_iter_rows_across_tables](/crates/oxide-library/tests/database_adapter/database_iter_rows_across_tables.md) |
| related | [database_read_row_by_pn](/crates/oxide-library/tests/database_adapter/database_read_row_by_pn.md) |
| related | [database_read_row_by_pn_404_maps_to_not_found](/crates/oxide-library/tests/database_adapter/database_read_row_by_pn_404_maps_to_not_found.md) |
| related | [database_update_row_modifies_payload](/crates/oxide-library/tests/database_adapter/database_update_row_modifies_payload.md) |
| related | [database_list_tables_returns_names](/crates/oxide-library/tests/database_adapter/database_list_tables_returns_names.md) |
| related | [database_read_table_returns_rows](/crates/oxide-library/tests/database_adapter/database_read_table_returns_rows.md) |
| related | [database_read_row_404_maps_to_not_found](/crates/oxide-library/tests/database_adapter/database_read_row_404_maps_to_not_found.md) |
| related | [wiremock](/_dependencies/cargo/wiremock.md) |
