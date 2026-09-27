---
okf_version: "0.2"
type: Function
title: read_row
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/read_row_1
language: rust
---

# read_row

## Signature

```rust
fn read_row(&self, table: &str, row_id: RowId) -> Result<ComponentRow, LibraryError>
```

## Source
Lines 270–276 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
