---
okf_version: "0.2"
type: Function
title: record_to_row
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/record_to_row
language: rust
---

# record_to_row

## Signature

```rust
pub(crate) fn record_to_row(record: &csv::StringRecord) -> Result<ComponentRow, LibraryError>
```

## Visibility

- `pub(crate)`

## Source
Lines 388–462 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [datasheet_from_cell](/crates/oxide-library/src/tables/datasheet_from_cell.md) |
| calls | [lifecycle_from_cell](/crates/oxide-library/src/tables/lifecycle_from_cell.md) |
| calls | [from_json_cell](/crates/oxide-library/src/tables/from_json_cell.md) |
| calls | [opt_primitive_from_cell](/crates/oxide-library/src/tables/opt_primitive_from_cell.md) |
| calls | [timestamp_from_cell](/crates/oxide-library/src/tables/timestamp_from_cell.md) |
| calls | [hash_from_cell](/crates/oxide-library/src/tables/hash_from_cell.md) |
| called_by | [library_row_to_component](/crates/oxide-library/src/adapters/local_git/helpers/library_row_to_component.md) |
| called_by | [read_table](/crates/oxide-library/src/tables/read_table.md) |
