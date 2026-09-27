---
okf_version: "0.2"
type: Function
title: database_iter_rows_across_tables
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
concept_id: crates/oxide-library/tests/database_adapter/database_iter_rows_across_tables
language: rust
---

# database_iter_rows_across_tables

[test]

## Signature

```rust
fn database_iter_rows_across_tables()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 355–404 in `crates/oxide-library/tests/database_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database_adapter](/crates/oxide-library/tests/database_adapter.md) |
| calls | [mk_row](/crates/oxide-library/tests/database_adapter/mk_row.md) |
| calls | [with_mock_server](/crates/oxide-library/tests/database_adapter/with_mock_server.md) |
| calls | [pin](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin.md) |
