---
okf_version: "0.2"
type: Function
title: pin
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin
language: rust
---

# pin

## Signature

```rust
fn pin(part_number: u8, orientation: PinOrientation, electrical: PinDirection) -> SymbolPin
```

## Source
Lines 17–23 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
| called_by | [pin_shape_style_uses_outside_edge_symbol_as_the_authoritative_slot](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_shape_style_uses_outside_edge_symbol_as_the_authoritative_slot.md) |
| called_by | [positions_carry_over_unchanged_no_y_flip](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/positions_carry_over_unchanged_no_y_flip.md) |
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
| called_by | [http_401_surfaces_auth_error](/crates/oxide-library/tests/distributor_digikey/http_401_surfaces_auth_error.md) |
| called_by | [lookup_by_mpn_with_inline_token](/crates/oxide-library/tests/distributor_digikey/lookup_by_mpn_with_inline_token.md) |
| called_by | [refresh_token_grant_calls_token_endpoint](/crates/oxide-library/tests/distributor_digikey/refresh_token_grant_calls_token_endpoint.md) |
| called_by | [cache_hit_short_circuits_post](/crates/oxide-library/tests/distributor_jlcpcb/cache_hit_short_circuits_post.md) |
| called_by | [lookup_by_mpn_handles_empty_list](/crates/oxide-library/tests/distributor_jlcpcb/lookup_by_mpn_handles_empty_list.md) |
| called_by | [lookup_by_mpn_parses_wiremock_fixture](/crates/oxide-library/tests/distributor_jlcpcb/lookup_by_mpn_parses_wiremock_fixture.md) |
| called_by | [cache_hit_short_circuits_network](/crates/oxide-library/tests/distributor_lcsc/cache_hit_short_circuits_network.md) |
| called_by | [http_429_surfaces_rate_limited_with_retry_after](/crates/oxide-library/tests/distributor_lcsc/http_429_surfaces_rate_limited_with_retry_after.md) |
| called_by | [lookup_by_mpn_parses_wiremock_fixture](/crates/oxide-library/tests/distributor_lcsc/lookup_by_mpn_parses_wiremock_fixture.md) |
| called_by | [lookup_by_mpn_returns_empty_on_empty_product_list](/crates/oxide-library/tests/distributor_lcsc/lookup_by_mpn_returns_empty_on_empty_product_list.md) |
| called_by | [empty_search_results_returns_empty_vec](/crates/oxide-library/tests/distributor_mouser/empty_search_results_returns_empty_vec.md) |
| called_by | [http_401_surfaces_auth_error](/crates/oxide-library/tests/distributor_mouser/http_401_surfaces_auth_error.md) |
| called_by | [lookup_by_mpn_passes_apikey_header](/crates/oxide-library/tests/distributor_mouser/lookup_by_mpn_passes_apikey_header.md) |
