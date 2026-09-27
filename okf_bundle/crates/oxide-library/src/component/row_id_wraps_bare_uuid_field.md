---
okf_version: "0.2"
type: Function
title: row_id_wraps_bare_uuid_field
description: "Verifies that `RowId` wraps cleanly around the row's `row_id` field —"
resource: crates/oxide-library/src/component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/component/row_id_wraps_bare_uuid_field
language: rust
---

# row_id_wraps_bare_uuid_field

Verifies that `RowId` wraps cleanly around the row's `row_id` field —

## Signature

```rust
fn row_id_wraps_bare_uuid_field()
```

## Decorators

- `test`

## Docstring

Verifies that `RowId` wraps cleanly around the row's `row_id` field —
the field is bare `Uuid` for serde-shape compatibility with the plan's
schema test, but consumers can `RowId::from_uuid(row.row_id)` to
get the typed wrapper when needed.
[test]

## Source
Lines 301–305 in `crates/oxide-library/src/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/oxide-library/src/component.md) |
| calls | [fixture_row](/crates/oxide-library/src/component/fixture_row.md) |
