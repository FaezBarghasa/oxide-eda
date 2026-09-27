---
okf_version: "0.2"
type: Function
title: append_row_grows_the_file
description: "`append_row` honours an empty file, then keeps growing it — and"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/append_row_grows_the_file
language: rust
---

# append_row_grows_the_file

`append_row` honours an empty file, then keeps growing it — and

## Signature

```rust
fn append_row_grows_the_file()
```

## Decorators

- `test`

## Docstring

`append_row` honours an empty file, then keeps growing it — and
leaves no stranded `.tmp` sibling behind.
[test]

## Source
Lines 612–622 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [mk_row](/crates/oxide-library/src/tables/mk_row.md) |
| calls | [append_row](/crates/oxide-library/src/tables/append_row.md) |
| calls | [read_table](/crates/oxide-library/src/tables/read_table.md) |
