---
okf_version: "0.2"
type: Function
title: insert_row
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter/insert_row
language: rust
---

# insert_row

## Signature

```rust
impl LocalGitAdapter { fn insert_row(&self, table: &str, row: ComponentRow, msg: &str) -> Result<(), LibraryError> }
```

## Source
Lines 290–312 in `crates/oxide-library/src/adapters/local_git/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapters/local_git/adapter.md) |
| calls | [component_to_library_row](/crates/oxide-library/src/adapters/local_git/helpers/component_to_library_row.md) |
| calls | [legacy_columns](/crates/oxide-library/src/adapters/local_git/helpers/legacy_columns.md) |
| calls | [validate_legacy_header](/crates/oxide-library/src/adapters/local_git/helpers/validate_legacy_header.md) |
