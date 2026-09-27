---
okf_version: "0.2"
type: Function
title: rename_table
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/rename_table_1
language: rust
---

# rename_table

## Signature

```rust
fn rename_table(&self, old: &str, new: &str, msg: &str) -> Result<(), LibraryError>
```

## Source
Lines 42–82 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
