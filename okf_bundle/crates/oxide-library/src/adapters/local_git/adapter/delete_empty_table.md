---
okf_version: "0.2"
type: Function
title: delete_empty_table
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/delete_empty_table
language: rust
---

# delete_empty_table

## Signature

```rust
impl LocalGitAdapter { fn delete_empty_table(&self, name: &str, msg: &str) -> Result<(), LibraryError> }
```

## Source
Lines 84–114 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
