---
okf_version: "0.2"
type: Function
title: get_symbol_round_trips_through_get_symbols_uuid
description: "[test]"
resource: crates/oxide-library/tests/database_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/database_adapter/get_symbol_round_trips_through_get_symbols_uuid
language: rust
---

# get_symbol_round_trips_through_get_symbols_uuid

[test]

## Signature

```rust
fn get_symbol_round_trips_through_get_symbols_uuid()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 72–101 in `crates/oxide-library/tests/database_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database_adapter](/crates/oxide-library/tests/database_adapter.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/database_adapter/fixture_symbol.md) |
| calls | [with_mock_server](/crates/oxide-library/tests/database_adapter/with_mock_server.md) |
| calls | [pin](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin.md) |
