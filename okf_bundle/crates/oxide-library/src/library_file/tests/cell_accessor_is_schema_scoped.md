---
okf_version: "0.2"
type: Function
title: cell_accessor_is_schema_scoped
description: "`LibraryTable::cell` returns `None` for out-of-schema columns even"
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/cell_accessor_is_schema_scoped
language: rust
---

# cell_accessor_is_schema_scoped

`LibraryTable::cell` returns `None` for out-of-schema columns even

## Signature

```rust
fn cell_accessor_is_schema_scoped()
```

## Decorators

- `test`

## Docstring

`LibraryTable::cell` returns `None` for out-of-schema columns even
when the row's `cells` map has an entry — protects against silent
schema-drift bugs where a row is written under a column the table
doesn't declare.
[test]

## Source
Lines 374–386 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
