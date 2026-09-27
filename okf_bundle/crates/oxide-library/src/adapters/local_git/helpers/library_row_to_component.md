---
okf_version: "0.2"
type: Function
title: library_row_to_component
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/library_row_to_component
language: rust
---

# library_row_to_component

## Signature

```rust
pub(super) fn library_row_to_component(row: &LibraryRow) -> Result<ComponentRow, LibraryError>
```

## Visibility

- `pub(super)`

## Source
Lines 114–121 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| calls | [record_to_row](/crates/oxide-library/src/tables/record_to_row.md) |
| called_by | [snapshot_table](/crates/oxide-library/src/adapters/local_git/primitives/snapshot_table.md) |
