---
okf_version: "0.2"
type: Function
title: with_mock_server
description: "Spin up a wiremock `MockServer` on a private runtime, run the setup"
resource: crates/oxide-library/tests/database_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/database_adapter/with_mock_server
language: rust
---

# with_mock_server

Spin up a wiremock `MockServer` on a private runtime, run the setup

## Signature

```rust
fn with_mock_server(setup: S, test: T)
```

## Type Parameters

- `S`
- `T`

## Docstring

Spin up a wiremock `MockServer` on a private runtime, run the setup
closure to register expectations, then hand the adapter (built against
the server URL) to the synchronous test body on a fresh thread.

## Source
Lines 37–58 in `crates/oxide-library/tests/database_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database_adapter](/crates/oxide-library/tests/database_adapter.md) |
| called_by | [database_iter_rows_across_tables](/crates/oxide-library/tests/database_adapter/database_iter_rows_across_tables.md) |
| called_by | [database_list_tables_returns_names](/crates/oxide-library/tests/database_adapter/database_list_tables_returns_names.md) |
| called_by | [database_read_row_404_maps_to_not_found](/crates/oxide-library/tests/database_adapter/database_read_row_404_maps_to_not_found.md) |
| called_by | [database_read_row_by_pn](/crates/oxide-library/tests/database_adapter/database_read_row_by_pn.md) |
| called_by | [database_read_row_by_pn_404_maps_to_not_found](/crates/oxide-library/tests/database_adapter/database_read_row_by_pn_404_maps_to_not_found.md) |
| called_by | [database_read_table_returns_rows](/crates/oxide-library/tests/database_adapter/database_read_table_returns_rows.md) |
| called_by | [database_round_trip_row](/crates/oxide-library/tests/database_adapter/database_round_trip_row.md) |
| called_by | [database_update_row_modifies_payload](/crates/oxide-library/tests/database_adapter/database_update_row_modifies_payload.md) |
| called_by | [get_symbol_404_maps_to_not_found](/crates/oxide-library/tests/database_adapter/get_symbol_404_maps_to_not_found.md) |
| called_by | [get_symbol_round_trips_through_get_symbols_uuid](/crates/oxide-library/tests/database_adapter/get_symbol_round_trips_through_get_symbols_uuid.md) |
| called_by | [list_symbols_round_trips_through_get_symbols](/crates/oxide-library/tests/database_adapter/list_symbols_round_trips_through_get_symbols.md) |
| called_by | [save_sim_posts_to_sims_route](/crates/oxide-library/tests/database_adapter/save_sim_posts_to_sims_route.md) |
| called_by | [save_symbol_posts_to_symbols_with_message_header](/crates/oxide-library/tests/database_adapter/save_symbol_posts_to_symbols_with_message_header.md) |
