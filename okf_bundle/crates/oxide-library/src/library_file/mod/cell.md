---
okf_version: "0.2"
type: Function
title: cell
description: "Look up `column` in `row.cells`, scoped to this table's schema."
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/cell
language: rust
---

# cell

Look up `column` in `row.cells`, scoped to this table's schema.

## Signature

```rust
impl LibraryTable { pub fn cell(&self, row: &'a LibraryRow, column: &str) -> Option<&'a str> }
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Look up `column` in `row.cells`, scoped to this table's schema.
Returns `None` if the column isn't in the table's header — even
if `row` happens to have an entry under that key.

## Source
Lines 241–247 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
