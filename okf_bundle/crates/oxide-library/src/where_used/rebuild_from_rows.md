---
okf_version: "0.2"
type: Function
title: rebuild_from_rows
description: "Rebuild the `primitive_to_rows` reverse index from a row scan"
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/rebuild_from_rows
language: rust
---

# rebuild_from_rows

Rebuild the `primitive_to_rows` reverse index from a row scan

## Signature

```rust
impl WhereUsedIndex { pub fn rebuild_from_rows(&mut self, rows: &[(String, ComponentRow)]) }
```

## Visibility

- `pub`

## Docstring

Rebuild the `primitive_to_rows` reverse index from a row scan
(`(table_name, row)` tuples — table_name is currently unused but
kept on the API so future per-table filters don't change the
signature).

Per the plan, every adapter exposes `iter_rows()` returning the same
shape. Rebuild is called on adapter open and after any row write
that the consumer hasn't otherwise tracked through `ingest_row`.

## Source
Lines 127–132 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |
