---
okf_version: "0.2"
type: Function
title: database_update_row_modifies_payload
description: "`update_row` carries a fresh `updated_at` and a new payload — the"
resource: crates/oxide-library/tests/database_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/database_adapter/database_update_row_modifies_payload
language: rust
---

# database_update_row_modifies_payload

`update_row` carries a fresh `updated_at` and a new payload — the

## Signature

```rust
fn database_update_row_modifies_payload()
```

## Decorators

- `test`

## Docstring

`update_row` carries a fresh `updated_at` and a new payload — the
adapter just forwards the row to the server, so the test verifies that
the PUT body matches what the caller sent (different `updated` from
the original `created`) and that the route round-trips successfully.
[test]

## Source
Lines 490–531 in `crates/oxide-library/tests/database_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database_adapter](/crates/oxide-library/tests/database_adapter.md) |
| calls | [mk_row](/crates/oxide-library/tests/database_adapter/mk_row.md) |
| calls | [with_mock_server](/crates/oxide-library/tests/database_adapter/with_mock_server.md) |
| calls | [pin](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin.md) |
