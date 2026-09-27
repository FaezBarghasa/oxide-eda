---
okf_version: "0.2"
type: Function
title: append_row
description: Append a single row to the table file.
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/append_row
language: rust
---

# append_row

Append a single row to the table file.

## Signature

```rust
pub fn append_row(path: &Path, row: &ComponentRow) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

Append a single row to the table file.

**Cost:** This reads the entire table into memory and rewrites it with
the appended row. For bulk loads (>10 rows), use `write_table` directly.

## Source
Lines 168–172 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [read_table](/crates/oxide-library/src/tables/read_table.md) |
| calls | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| called_by | [append_row_grows_the_file](/crates/oxide-library/src/tables/append_row_grows_the_file.md) |
