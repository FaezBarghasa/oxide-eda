---
okf_version: "0.2"
type: Function
title: read_table
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/read_table_1
language: rust
---

# read_table

## Signature

```rust
fn read_table(&self, name: &str) -> Result<Vec<ComponentRow>, LibraryError>
```

## Source
Lines 312–326 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
