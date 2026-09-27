---
okf_version: "0.2"
type: Function
title: library_id_query
description: "`library_id` query string segment used by every row/table call. The"
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/library_id_query
language: rust
---

# library_id_query

`library_id` query string segment used by every row/table call. The

## Signature

```rust
impl DatabaseAdapter { fn library_id_query(&self) -> String }
```

## Docstring

`library_id` query string segment used by every row/table call. The
server keys row storage by `(library_id, table_name, row_id)`; a
missing or wrong id surfaces as a 404 from the route, which we map
to `NotFound` at the call site.

## Source
Lines 264–266 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
