---
okf_version: "0.2"
type: Function
title: header_only_tsv_is_empty_rows
description: "Header-only TSV (no body rows) parses to an empty `rows` vec."
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/header_only_tsv_is_empty_rows
language: rust
---

# header_only_tsv_is_empty_rows

Header-only TSV (no body rows) parses to an empty `rows` vec.

## Signature

```rust
fn header_only_tsv_is_empty_rows()
```

## Decorators

- `test`

## Docstring

Header-only TSV (no body rows) parses to an empty `rows` vec.
This is the shape an empty user-created table has on disk
before any rows are added.
[test]

## Source
Lines 188–204 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
