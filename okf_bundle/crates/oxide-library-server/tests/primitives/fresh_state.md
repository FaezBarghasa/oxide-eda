---
okf_version: "0.2"
type: Function
title: fresh_state
resource: crates/oxide-library-server/tests/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:43:54Z"
concept_id: crates/oxide-library-server/tests/primitives/fresh_state
language: rust
---

# fresh_state

## Signature

```rust
fn fresh_state() -> AppState
```

## Source
Lines 31–38 in `crates/oxide-library-server/tests/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library-server/tests/primitives.md) |
| calls | [ensure_test_token](/crates/oxide-library-server/tests/primitives/ensure_test_token.md) |
| called_by | [get_symbol_400_without_library_id](/crates/oxide-library-server/tests/primitives/get_symbol_400_without_library_id.md) |
| called_by | [get_symbol_404_when_unknown](/crates/oxide-library-server/tests/primitives/get_symbol_404_when_unknown.md) |
| called_by | [list_symbols_filters_by_library_id](/crates/oxide-library-server/tests/primitives/list_symbols_filters_by_library_id.md) |
| called_by | [post_then_get_footprint_round_trip](/crates/oxide-library-server/tests/primitives/post_then_get_footprint_round_trip.md) |
| called_by | [post_then_get_sim_round_trip](/crates/oxide-library-server/tests/primitives/post_then_get_sim_round_trip.md) |
| called_by | [post_then_get_symbol_round_trip](/crates/oxide-library-server/tests/primitives/post_then_get_symbol_round_trip.md) |
| called_by | [primitives_migration_creates_tables](/crates/oxide-library-server/tests/primitives/primitives_migration_creates_tables.md) |
