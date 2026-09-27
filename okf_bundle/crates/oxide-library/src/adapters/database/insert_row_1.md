---
okf_version: "0.2"
type: Function
title: insert_row
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/insert_row_1
language: rust
---

# insert_row

## Signature

```rust
fn insert_row(&self, table: &str, row: ComponentRow, msg: &str) -> Result<(), LibraryError>
```

## Source
Lines 377–408 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
