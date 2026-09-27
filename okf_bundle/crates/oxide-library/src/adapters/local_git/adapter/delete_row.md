---
okf_version: "0.2"
type: Function
title: delete_row
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/delete_row
language: rust
---

# delete_row

## Signature

```rust
impl LocalGitAdapter { fn delete_row(&self, table: &str, row_id: RowId, msg: &str) -> Result<(), LibraryError> }
```

## Source
Lines 344–367 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
