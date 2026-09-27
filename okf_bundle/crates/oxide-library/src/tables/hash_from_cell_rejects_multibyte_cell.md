---
okf_version: "0.2"
type: Function
title: hash_from_cell_rejects_multibyte_cell
description: A 64-BYTE cell whose bytes are not all ASCII hex is a recoverable
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/hash_from_cell_rejects_multibyte_cell
language: rust
---

# hash_from_cell_rejects_multibyte_cell

A 64-BYTE cell whose bytes are not all ASCII hex is a recoverable

## Signature

```rust
fn hash_from_cell_rejects_multibyte_cell()
```

## Decorators

- `test`

## Docstring

A 64-BYTE cell whose bytes are not all ASCII hex is a recoverable
error, never a char-boundary panic.
[test]

## Source
Lines 717–728 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
