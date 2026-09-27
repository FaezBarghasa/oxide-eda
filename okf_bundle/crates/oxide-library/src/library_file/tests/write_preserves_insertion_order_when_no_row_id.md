---
okf_version: "0.2"
type: Function
title: write_preserves_insertion_order_when_no_row_id
description: "Tables without a `row_id` column preserve insertion order — no"
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/write_preserves_insertion_order_when_no_row_id
language: rust
---

# write_preserves_insertion_order_when_no_row_id

Tables without a `row_id` column preserve insertion order — no

## Signature

```rust
fn write_preserves_insertion_order_when_no_row_id()
```

## Decorators

- `test`

## Docstring

Tables without a `row_id` column preserve insertion order — no
canonical key to sort by, so we don't reshuffle the user's
arbitrary lookup tables.
[test]

## Source
Lines 470–493 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
| calls | [fixture_manifest](/crates/oxide-library/src/library_file/tests/fixture_manifest.md) |
