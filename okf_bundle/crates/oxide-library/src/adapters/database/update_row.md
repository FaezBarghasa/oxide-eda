---
okf_version: "0.2"
type: Function
title: update_row
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/update_row
language: rust
---

# update_row

## Signature

```rust
impl DatabaseAdapter { fn update_row(&self, table: &str, row: ComponentRow, msg: &str) -> Result<(), LibraryError> }
```

## Source
Lines 410–444 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
