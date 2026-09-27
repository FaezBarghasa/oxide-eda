---
okf_version: "0.2"
type: Function
title: mk_row
description: "Build a `ComponentRow` fixture — same shape as `component::tests::fixture_row`"
resource: crates/oxide-library/tests/database_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/database_adapter/mk_row
language: rust
---

# mk_row

Build a `ComponentRow` fixture — same shape as `component::tests::fixture_row`

## Signature

```rust
fn mk_row(pn: &str, class: &str) -> ComponentRow
```

## Docstring

Build a `ComponentRow` fixture — same shape as `component::tests::fixture_row`
but with controllable PN + class so the assertions in each test don't
fight over identity.

## Source
Lines 244–271 in `crates/oxide-library/tests/database_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database_adapter](/crates/oxide-library/tests/database_adapter.md) |
| called_by | [database_iter_rows_across_tables](/crates/oxide-library/tests/database_adapter/database_iter_rows_across_tables.md) |
| called_by | [database_read_row_by_pn](/crates/oxide-library/tests/database_adapter/database_read_row_by_pn.md) |
| called_by | [database_read_table_returns_rows](/crates/oxide-library/tests/database_adapter/database_read_table_returns_rows.md) |
| called_by | [database_round_trip_row](/crates/oxide-library/tests/database_adapter/database_round_trip_row.md) |
| called_by | [database_update_row_modifies_payload](/crates/oxide-library/tests/database_adapter/database_update_row_modifies_payload.md) |
