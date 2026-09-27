---
okf_version: "0.2"
type: Function
title: column_type_token_round_trip_all_variants
description: Token-level round-trip for every variant — paranoia test that
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/column_type_token_round_trip_all_variants
language: rust
---

# column_type_token_round_trip_all_variants

Token-level round-trip for every variant — paranoia test that

## Signature

```rust
fn column_type_token_round_trip_all_variants()
```

## Decorators

- `test`

## Docstring

Token-level round-trip for every variant — paranoia test that
every enum variant survives `to_token` → `parse_token`.
[test]

## Source
Lines 627–646 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
