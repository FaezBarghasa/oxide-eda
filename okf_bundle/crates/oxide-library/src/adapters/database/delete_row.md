---
okf_version: "0.2"
type: Function
title: delete_row
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/delete_row
language: rust
---

# delete_row

## Signature

```rust
impl DatabaseAdapter { fn delete_row(&self, table: &str, row_id: RowId, msg: &str) -> Result<(), LibraryError> }
```

## Source
Lines 446–473 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
