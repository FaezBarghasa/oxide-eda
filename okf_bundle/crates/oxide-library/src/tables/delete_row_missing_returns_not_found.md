---
okf_version: "0.2"
type: Function
title: delete_row_missing_returns_not_found
description: "[test]"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/delete_row_missing_returns_not_found
language: rust
---

# delete_row_missing_returns_not_found

[test]

## Signature

```rust
fn delete_row_missing_returns_not_found()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 660–666 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [mk_row](/crates/oxide-library/src/tables/mk_row.md) |
| calls | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| calls | [delete_row](/crates/oxide-library/src/tables/delete_row.md) |
