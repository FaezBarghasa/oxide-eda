---
okf_version: "0.2"
type: Function
title: update_row
description: "Replace the row whose `row_id` matches. Returns `NotFound` if no such"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/update_row
language: rust
---

# update_row

Replace the row whose `row_id` matches. Returns `NotFound` if no such

## Signature

```rust
pub fn update_row(path: &Path, row: &ComponentRow) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

Replace the row whose `row_id` matches. Returns `NotFound` if no such
row exists in the table.

## Source
Lines 192–210 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [read_table](/crates/oxide-library/src/tables/read_table.md) |
| calls | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| called_by | [update_row_missing_returns_not_found](/crates/oxide-library/src/tables/update_row_missing_returns_not_found.md) |
| called_by | [update_row_replaces_in_place](/crates/oxide-library/src/tables/update_row_replaces_in_place.md) |
