---
okf_version: "0.2"
type: Function
title: delete_row
description: "Remove the row whose `row_id` matches. Returns `NotFound` if no such row"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/delete_row
language: rust
---

# delete_row

Remove the row whose `row_id` matches. Returns `NotFound` if no such row

## Signature

```rust
pub fn delete_row(path: &Path, row_id: RowId) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

Remove the row whose `row_id` matches. Returns `NotFound` if no such row
exists in the table.

## Source
Lines 176–188 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [read_table](/crates/oxide-library/src/tables/read_table.md) |
| calls | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| called_by | [delete_row_missing_returns_not_found](/crates/oxide-library/src/tables/delete_row_missing_returns_not_found.md) |
| called_by | [delete_row_removes_only_matching_id](/crates/oxide-library/src/tables/delete_row_removes_only_matching_id.md) |
