---
okf_version: "0.2"
type: Function
title: read_table
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/read_table
language: rust
---

# read_table

## Signature

```rust
impl LocalGitAdapter { fn read_table(&self, name: &str) -> Result<Vec<ComponentRow>, LibraryError> }
```

## Source
Lines 256–258 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
