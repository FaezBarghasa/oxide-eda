---
okf_version: "0.2"
type: Function
title: read_table
description: "Read every row from `path`. Empty file (header-only) yields an empty `Vec`."
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/read_table
language: rust
---

# read_table

Read every row from `path`. Empty file (header-only) yields an empty `Vec`.

## Signature

```rust
pub fn read_table(path: &Path) -> Result<Vec<ComponentRow>, LibraryError>
```

## Visibility

- `pub`

## Docstring

Read every row from `path`. Empty file (header-only) yields an empty `Vec`.

Errors: `Io` on I/O failure, `Backend` on a malformed cell or schema
mismatch.

## Source
Lines 92–136 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [record_to_row](/crates/oxide-library/src/tables/record_to_row.md) |
| called_by | [append_row](/crates/oxide-library/src/tables/append_row.md) |
| called_by | [append_row_grows_the_file](/crates/oxide-library/src/tables/append_row_grows_the_file.md) |
| called_by | [delete_row](/crates/oxide-library/src/tables/delete_row.md) |
| called_by | [delete_row_removes_only_matching_id](/crates/oxide-library/src/tables/delete_row_removes_only_matching_id.md) |
| called_by | [none_primitive_refs_encode_empty](/crates/oxide-library/src/tables/none_primitive_refs_encode_empty.md) |
| called_by | [read_missing_file_is_empty](/crates/oxide-library/src/tables/read_missing_file_is_empty.md) |
| called_by | [tsv_roundtrip_preserves_row](/crates/oxide-library/src/tables/tsv_roundtrip_preserves_row.md) |
| called_by | [update_row](/crates/oxide-library/src/tables/update_row.md) |
| called_by | [update_row_replaces_in_place](/crates/oxide-library/src/tables/update_row_replaces_in_place.md) |
