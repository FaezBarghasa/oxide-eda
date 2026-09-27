---
okf_version: "0.2"
type: Function
title: column_type_parse_rejects_empty_enum
description: "`ColumnType::Enum` requires at least one non-empty value —"
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/column_type_parse_rejects_empty_enum
language: rust
---

# column_type_parse_rejects_empty_enum

`ColumnType::Enum` requires at least one non-empty value —

## Signature

```rust
fn column_type_parse_rejects_empty_enum()
```

## Decorators

- `test`

## Docstring

`ColumnType::Enum` requires at least one non-empty value —
`enum:` (empty) or `enum:,` (only blanks) are rejected.
[test]

## Source
Lines 617–622 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
