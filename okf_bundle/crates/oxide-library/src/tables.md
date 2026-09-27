---
okf_version: "0.2"
type: Module
title: tables
description: TSV reader/writer for component tables (Altium DBLib model).
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables
language: rust
---

# tables

TSV reader/writer for component tables (Altium DBLib model).

## Docstring

TSV reader/writer for component tables (Altium DBLib model).

Per `v0.9-refactor-2-plan.md` §2.4, every adapter (LocalGit, Database,
future flavours) shares one column schema:

```text
row_id  internal_pn  class  datasheet  state  symbol_ref  footprint_ref
sim_ref  primary_mpn  alternates  supply  parameters  pin_map_overrides
created  updated  content_hash
```

Scalar columns (`row_id`, `internal_pn`, `class`, `state`, `created`,
`updated`, `content_hash`) are written as plain strings. Nested columns
(`PrimitiveRef`, `ManufacturerPart`, `Vec<…>`, `ParamMap`, `PlmReserved`)
are JSON-encoded inside the cell.

`Option<PrimitiveRef>` cells: empty string = `None`, JSON = `Some`.

This module is the *file format*; commit / branch handling lives
in [`crate::adapters::local_git::LocalGitAdapter`]. The unit
tests here only exercise the TSV serialisation contract.

## Relationships

| Type | Target |
|------|--------|
| related | [TableSchema](/crates/oxide-library/src/tables/TableSchema.md) |
| related | [read_table](/crates/oxide-library/src/tables/read_table.md) |
| related | [write_table](/crates/oxide-library/src/tables/write_table.md) |
| related | [append_row](/crates/oxide-library/src/tables/append_row.md) |
| related | [delete_row](/crates/oxide-library/src/tables/delete_row.md) |
| related | [update_row](/crates/oxide-library/src/tables/update_row.md) |
| related | [json_cell](/crates/oxide-library/src/tables/json_cell.md) |
| related | [from_json_cell](/crates/oxide-library/src/tables/from_json_cell.md) |
| related | [datasheet_to_cell](/crates/oxide-library/src/tables/datasheet_to_cell.md) |
| related | [datasheet_from_cell](/crates/oxide-library/src/tables/datasheet_from_cell.md) |
| related | [lifecycle_to_cell](/crates/oxide-library/src/tables/lifecycle_to_cell.md) |
| related | [lifecycle_from_cell](/crates/oxide-library/src/tables/lifecycle_from_cell.md) |
| related | [opt_primitive_to_cell](/crates/oxide-library/src/tables/opt_primitive_to_cell.md) |
| related | [opt_primitive_from_cell](/crates/oxide-library/src/tables/opt_primitive_from_cell.md) |
| related | [timestamp_to_cell](/crates/oxide-library/src/tables/timestamp_to_cell.md) |
| related | [timestamp_from_cell](/crates/oxide-library/src/tables/timestamp_from_cell.md) |
| related | [hash_to_cell](/crates/oxide-library/src/tables/hash_to_cell.md) |
| related | [hash_from_cell](/crates/oxide-library/src/tables/hash_from_cell.md) |
| related | [row_to_record](/crates/oxide-library/src/tables/row_to_record.md) |
| related | [record_to_row](/crates/oxide-library/src/tables/record_to_row.md) |
| related | [has_stray_tmp](/crates/oxide-library/src/tables/has_stray_tmp.md) |
| related | [DenyNewFiles](/crates/oxide-library/src/tables/DenyNewFiles.md) |
| related | [on](/crates/oxide-library/src/tables/on.md) |
| related | [settle_deny](/crates/oxide-library/src/tables/settle_deny.md) |
| related | [on](/crates/oxide-library/src/tables/on.md) |
| related | [settle_deny](/crates/oxide-library/src/tables/settle_deny.md) |
| related | [drop](/crates/oxide-library/src/tables/drop.md) |
| related | [drop](/crates/oxide-library/src/tables/drop.md) |
| related | [mk_row](/crates/oxide-library/src/tables/mk_row.md) |
| related | [tsv_roundtrip_preserves_row](/crates/oxide-library/src/tables/tsv_roundtrip_preserves_row.md) |
| related | [read_missing_file_is_empty](/crates/oxide-library/src/tables/read_missing_file_is_empty.md) |
| related | [append_row_grows_the_file](/crates/oxide-library/src/tables/append_row_grows_the_file.md) |
| related | [write_table_is_atomic_and_preserves_original_on_failure](/crates/oxide-library/src/tables/write_table_is_atomic_and_preserves_original_on_failure.md) |
| related | [delete_row_removes_only_matching_id](/crates/oxide-library/src/tables/delete_row_removes_only_matching_id.md) |
| related | [delete_row_missing_returns_not_found](/crates/oxide-library/src/tables/delete_row_missing_returns_not_found.md) |
| related | [update_row_replaces_in_place](/crates/oxide-library/src/tables/update_row_replaces_in_place.md) |
| related | [update_row_missing_returns_not_found](/crates/oxide-library/src/tables/update_row_missing_returns_not_found.md) |
| related | [none_primitive_refs_encode_empty](/crates/oxide-library/src/tables/none_primitive_refs_encode_empty.md) |
| related | [hash_round_trip_preserves_bytes](/crates/oxide-library/src/tables/hash_round_trip_preserves_bytes.md) |
| related | [hash_from_cell_rejects_multibyte_cell](/crates/oxide-library/src/tables/hash_from_cell_rejects_multibyte_cell.md) |
| related | [schema_constant_matches_header_length](/crates/oxide-library/src/tables/schema_constant_matches_header_length.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
