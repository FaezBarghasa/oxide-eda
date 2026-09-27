---
okf_version: "0.2"
type: Module
title: primitives
description: Integration tests for the primitive routes
resource: crates/oxide-library-server/tests/primitives.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:43:54Z"
concept_id: crates/oxide-library-server/tests/primitives
language: rust
---

# primitives

Integration tests for the primitive routes

## Docstring

Integration tests for the primitive routes
(`/symbols` / `/footprints` / `/sims`).

Each test exercises a `POST` → `GET` round-trip via actix-web test harness
against the in-memory test harness, exactly mirroring the flow that the
`LibraryAdapter` will use in production. Auth is the same fixture bearer
token used by `tests/integration_db.rs`.

## Relationships

| Type | Target |
|------|--------|
| related | [ensure_test_token](/crates/oxide-library-server/tests/primitives/ensure_test_token.md) |
| related | [bearer_header](/crates/oxide-library-server/tests/primitives/bearer_header.md) |
| related | [fresh_state](/crates/oxide-library-server/tests/primitives/fresh_state.md) |
| related | [primitives_migration_creates_tables](/crates/oxide-library-server/tests/primitives/primitives_migration_creates_tables.md) |
| related | [post_then_get_symbol_round_trip](/crates/oxide-library-server/tests/primitives/post_then_get_symbol_round_trip.md) |
| related | [list_symbols_filters_by_library_id](/crates/oxide-library-server/tests/primitives/list_symbols_filters_by_library_id.md) |
| related | [post_then_get_footprint_round_trip](/crates/oxide-library-server/tests/primitives/post_then_get_footprint_round_trip.md) |
| related | [post_then_get_sim_round_trip](/crates/oxide-library-server/tests/primitives/post_then_get_sim_round_trip.md) |
| related | [get_symbol_404_when_unknown](/crates/oxide-library-server/tests/primitives/get_symbol_404_when_unknown.md) |
| related | [get_symbol_400_without_library_id](/crates/oxide-library-server/tests/primitives/get_symbol_400_without_library_id.md) |
