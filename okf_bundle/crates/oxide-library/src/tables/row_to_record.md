---
okf_version: "0.2"
type: Function
title: row_to_record
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/row_to_record
language: rust
---

# row_to_record

## Signature

```rust
pub(crate) fn row_to_record(row: &ComponentRow) -> Result<Vec<String>, LibraryError>
```

## Visibility

- `pub(crate)`

## Source
Lines 321–386 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [json_cell](/crates/oxide-library/src/tables/json_cell.md) |
| called_by | [component_to_library_row](/crates/oxide-library/src/adapters/local_git/helpers/component_to_library_row.md) |
| called_by | [write_table](/crates/oxide-library/src/tables/write_table.md) |
