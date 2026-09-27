---
okf_version: "0.2"
type: Function
title: write_table
description: "Replace the contents of `path` with `rows`. Creates parent directories"
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/write_table
language: rust
---

# write_table

Replace the contents of `path` with `rows`. Creates parent directories

## Signature

```rust
pub fn write_table(path: &Path, rows: &[ComponentRow]) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

Replace the contents of `path` with `rows`. Creates parent directories
if they don't exist.

Crash-safe: the bytes land via [`oxide_types::atomic_io::atomic_write`]
(temp file + fsync + rename), so a crash or power loss mid-save leaves
either the previous table or the new one — never a truncated file. Every
mutator here ([`append_row`], [`delete_row`], [`update_row`]) rewrites the
whole table through this one funnel, so they inherit the guarantee.

## Source
Lines 146–162 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [row_to_record](/crates/oxide-library/src/tables/row_to_record.md) |
| called_by | [append_row](/crates/oxide-library/src/tables/append_row.md) |
| called_by | [delete_row](/crates/oxide-library/src/tables/delete_row.md) |
| called_by | [delete_row_missing_returns_not_found](/crates/oxide-library/src/tables/delete_row_missing_returns_not_found.md) |
| called_by | [delete_row_removes_only_matching_id](/crates/oxide-library/src/tables/delete_row_removes_only_matching_id.md) |
| called_by | [none_primitive_refs_encode_empty](/crates/oxide-library/src/tables/none_primitive_refs_encode_empty.md) |
| called_by | [tsv_roundtrip_preserves_row](/crates/oxide-library/src/tables/tsv_roundtrip_preserves_row.md) |
| called_by | [update_row](/crates/oxide-library/src/tables/update_row.md) |
| called_by | [update_row_replaces_in_place](/crates/oxide-library/src/tables/update_row_replaces_in_place.md) |
| called_by | [write_table_is_atomic_and_preserves_original_on_failure](/crates/oxide-library/src/tables/write_table_is_atomic_and_preserves_original_on_failure.md) |
| called_by | [full_row_round_trip_via_tsv](/crates/oxide-library/tests/foundation_round_trip/full_row_round_trip_via_tsv.md) |
