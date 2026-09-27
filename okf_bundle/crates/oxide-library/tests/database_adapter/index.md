# database_adapter

## Functions

- [database_iter_rows_across_tables](database_iter_rows_across_tables.md) — [test]
- [database_list_tables_returns_names](database_list_tables_returns_names.md) — [test]
- [database_read_row_404_maps_to_not_found](database_read_row_404_maps_to_not_found.md) — [test]
- [database_read_row_by_pn](database_read_row_by_pn.md) — [test]
- [database_read_row_by_pn_404_maps_to_not_found](database_read_row_by_pn_404_maps_to_not_found.md) — [test]
- [database_read_table_returns_rows](database_read_table_returns_rows.md) — [test]
- [database_round_trip_row](database_round_trip_row.md) — Round-trip: insert → read → update → delete, each call hitting its own
- [database_update_row_modifies_payload](database_update_row_modifies_payload.md) — `update_row` carries a fresh `updated_at` and a new payload — the
- [fixture_symbol](fixture_symbol.md) — ── Primitive CRUD over HTTP ─────────────────────────────────────────────
- [get_symbol_404_maps_to_not_found](get_symbol_404_maps_to_not_found.md) — [test]
- [get_symbol_round_trips_through_get_symbols_uuid](get_symbol_round_trips_through_get_symbols_uuid.md) — [test]
- [list_symbols_round_trips_through_get_symbols](list_symbols_round_trips_through_get_symbols.md) — [test]
- [mk_row](mk_row.md) — Build a `ComponentRow` fixture — same shape as `component::tests::fixture_row`
- [save_sim_posts_to_sims_route](save_sim_posts_to_sims_route.md) — [test]
- [save_symbol_posts_to_symbols_with_message_header](save_symbol_posts_to_symbols_with_message_header.md) — [test]
- [with_mock_server](with_mock_server.md) — Spin up a wiremock `MockServer` on a private runtime, run the setup
