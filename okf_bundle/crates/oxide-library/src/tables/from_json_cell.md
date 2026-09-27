---
okf_version: "0.2"
type: Function
title: from_json_cell
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/from_json_cell
language: rust
---

# from_json_cell

## Signature

```rust
fn from_json_cell(s: &str, name: &str) -> Result<T, LibraryError>
```

## Type Parameters

- `T: DeserializeOwned`

## Source
Lines 218–220 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| called_by | [datasheet_from_cell](/crates/oxide-library/src/tables/datasheet_from_cell.md) |
| called_by | [opt_primitive_from_cell](/crates/oxide-library/src/tables/opt_primitive_from_cell.md) |
| called_by | [record_to_row](/crates/oxide-library/src/tables/record_to_row.md) |
