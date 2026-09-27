---
okf_version: "0.2"
type: Function
title: read_row
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/read_row
language: rust
---

# read_row

## Signature

```rust
impl DatabaseAdapter { fn read_row(&self, table: &str, row_id: RowId) -> Result<ComponentRow, LibraryError> }
```

## Source
Lines 343–364 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
