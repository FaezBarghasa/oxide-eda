---
okf_version: "0.2"
type: Function
title: iter_rows
description: "Composed from `list_tables` + `read_table` per plan §9 (the server"
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/iter_rows
language: rust
---

# iter_rows

Composed from `list_tables` + `read_table` per plan §9 (the server

## Signature

```rust
impl DatabaseAdapter { fn iter_rows(&self) -> Result<Vec<(String, ComponentRow)>, LibraryError> }
```

## Docstring

Composed from `list_tables` + `read_table` per plan §9 (the server
only ships the 6 row/table routes; no aggregate `/rows` endpoint).
Cost is one round-trip per table, then one per non-empty table —
fine at v0.9 scale.

## Source
Lines 332–341 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
