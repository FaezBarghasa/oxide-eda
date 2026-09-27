---
okf_version: "0.2"
type: Function
title: untyped_tables_skip_column_types_block
description: Untyped tables round-trip without emitting an empty
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/untyped_tables_skip_column_types_block
language: rust
---

# untyped_tables_skip_column_types_block

Untyped tables round-trip without emitting an empty

## Signature

```rust
fn untyped_tables_skip_column_types_block()
```

## Decorators

- `test`

## Docstring

Untyped tables round-trip without emitting an empty
`[tables.<name>.column_types]` block — keeps the `.snxlib`
clean for the common "user just made a quick lookup table" case.
[test]

## Source
Lines 548–562 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
| calls | [fixture_table_resistors](/crates/oxide-library/src/library_file/tests/fixture_table_resistors.md) |
| calls | [fixture_manifest](/crates/oxide-library/src/library_file/tests/fixture_manifest.md) |
