---
okf_version: "0.2"
type: Function
title: legacy_file_reconciles_part_count
description: "Legacy `.snxsym` files were written before `part_count` existed, so"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/legacy_file_reconciles_part_count
language: rust
---

# legacy_file_reconciles_part_count

Legacy `.snxsym` files were written before `part_count` existed, so

## Signature

```rust
fn legacy_file_reconciles_part_count()
```

## Decorators

- `test`

## Docstring

Legacy `.snxsym` files were written before `part_count` existed, so
they load with the serde default (`1`) even when pins live on higher
parts. The loader must reconcile the declared count upward from the
highest pin `part_number` so no populated unit is lost.
[test]

## Source
Lines 427–440 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
