---
okf_version: "0.2"
type: Function
title: rows_for_primitive
description: All rows that reference the given primitive. Returned slice is empty
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/rows_for_primitive_1
language: rust
---

# rows_for_primitive

All rows that reference the given primitive. Returned slice is empty

## Signature

```rust
pub fn rows_for_primitive(&self, r: &PrimitiveRef) -> &[RowId]
```

## Visibility

- `pub`

## Docstring

All rows that reference the given primitive. Returned slice is empty
when the primitive isn't referenced.

## Source
Lines 164–169 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |
