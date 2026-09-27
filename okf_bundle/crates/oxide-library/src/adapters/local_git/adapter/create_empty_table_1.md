---
okf_version: "0.2"
type: Function
title: create_empty_table
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/create_empty_table_1
language: rust
---

# create_empty_table

## Signature

```rust
fn create_empty_table(&self, name: &str, msg: &str) -> Result<(), LibraryError>
```

## Source
Lines 213–254 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
| calls | [legacy_columns](/crates/oxide-library/src/adapters/local_git/helpers/legacy_columns.md) |
