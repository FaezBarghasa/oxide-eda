---
okf_version: "0.2"
type: Function
title: legacy_columns
description: "Header expected for the v0.9 fixed-schema `[tables.<name>]` blocks"
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/legacy_columns
language: rust
---

# legacy_columns

Header expected for the v0.9 fixed-schema `[tables.<name>]` blocks

## Signature

```rust
pub(super) fn legacy_columns() -> Vec<String>
```

## Visibility

- `pub(super)`

## Docstring

Header expected for the v0.9 fixed-schema `[tables.<name>]` blocks
— same column ordering as the pre-refactor `tables/*.tsv` files
so the conversion through [`row_to_record`] / [`record_to_row`]
stays bit-exact. Stage 12 lifts the fixed-schema constraint.

## Source
Lines 79–81 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [create_empty_table](/crates/oxide-library/src/adapters/local_git/adapter/create_empty_table.md) |
| called_by | [insert_row](/crates/oxide-library/src/adapters/local_git/adapter/insert_row.md) |
