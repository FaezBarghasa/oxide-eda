---
okf_version: "0.2"
type: Function
title: parse_rejects_column_type_for_unknown_column
description: "`column_types` with a key that doesn't exist in the TSV header"
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/parse_rejects_column_type_for_unknown_column
language: rust
---

# parse_rejects_column_type_for_unknown_column

`column_types` with a key that doesn't exist in the TSV header

## Signature

```rust
fn parse_rejects_column_type_for_unknown_column()
```

## Decorators

- `test`

## Docstring

`column_types` with a key that doesn't exist in the TSV header
is loud — silently rendering dropdowns for ghost columns would
be confusing.
[test]

## Source
Lines 568–589 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
