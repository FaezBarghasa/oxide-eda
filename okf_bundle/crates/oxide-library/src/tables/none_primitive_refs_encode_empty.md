---
okf_version: "0.2"
type: Function
title: none_primitive_refs_encode_empty
description: "`none` footprint and sim refs encode as empty strings; round-trip"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/none_primitive_refs_encode_empty
language: rust
---

# none_primitive_refs_encode_empty

`none` footprint and sim refs encode as empty strings; round-trip

## Signature

```rust
fn none_primitive_refs_encode_empty()
```

## Decorators

- `test`

## Docstring

`none` footprint and sim refs encode as empty strings; round-trip
preserves the `None` shape.
[test]

## Source
Lines 692–700 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [mk_row](/crates/oxide-library/src/tables/mk_row.md) |
| calls | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| calls | [read_table](/crates/oxide-library/src/tables/read_table.md) |
