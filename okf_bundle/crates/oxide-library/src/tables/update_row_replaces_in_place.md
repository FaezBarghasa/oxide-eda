---
okf_version: "0.2"
type: Function
title: update_row_replaces_in_place
description: "`update_row` replaces a row in-place by id."
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/update_row_replaces_in_place
language: rust
---

# update_row_replaces_in_place

`update_row` replaces a row in-place by id.

## Signature

```rust
fn update_row_replaces_in_place()
```

## Decorators

- `test`

## Docstring

`update_row` replaces a row in-place by id.
[test]

## Source
Lines 670–679 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [mk_row](/crates/oxide-library/src/tables/mk_row.md) |
| calls | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| calls | [update_row](/crates/oxide-library/src/tables/update_row.md) |
| calls | [read_table](/crates/oxide-library/src/tables/read_table.md) |
