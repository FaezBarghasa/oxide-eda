---
okf_version: "0.2"
type: Module
title: library_file
description: "`LibraryFile` — on-disk representation of a `.snxlib` file."
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod
language: rust
---

# library_file

`LibraryFile` — on-disk representation of a `.snxlib` file.

## Docstring

`LibraryFile` — on-disk representation of a `.snxlib` file.

Per `v0.9-snxlib-as-file-plan.md` §1, a Oxide component library is a
directory whose `.snxlib` file is the user-facing entry point. The file
is a TOML document combining a small manifest header with one
`[tables.<name>]` block per user-defined component category. Each table
holds raw TSV inside a TOML literal multi-line string so the data is
line-diffable in git and editable in any spreadsheet.

Stage 1 (this module) lands the storage shape only:
* [`SnxlibManifest`] — the manifest header (everything outside the
`[tables.*]` blocks). `library_id` lives at the document root, not
inside `[library]`, matching the v0.9 format.
* [`LibraryTable`] / [`LibraryRow`] — the parsed in-memory view of
one `[tables.<name>]` block. The TSV header row defines the schema;
columns are user-defined per table.
* [`LibraryFile`] — the top-level type combining a manifest header
with a map of parsed tables. [`LibraryFile::parse`] and
[`LibraryFile::write`] are the round-trip entrypoints.

Reserved-core-column validation, typed accessors for fields like
`row_id` / `version` / `released`, and the cascade engine are later
stages of `v0.9-snxlib-as-file-plan.md` — Stage 1 only establishes
the storage shape and the round-trip contract.

## Relationships

| Type | Target |
|------|--------|
| related | [LibraryFile](/crates/oxide-library/src/library_file/mod/LibraryFile.md) |
| related | [SnxlibManifest](/crates/oxide-library/src/library_file/mod/SnxlibManifest.md) |
| related | [ClassEntry](/crates/oxide-library/src/library_file/mod/ClassEntry.md) |
| related | [LibrarySection](/crates/oxide-library/src/library_file/mod/LibrarySection.md) |
| related | [LibraryTable](/crates/oxide-library/src/library_file/mod/LibraryTable.md) |
| related | [ColumnType](/crates/oxide-library/src/library_file/mod/ColumnType.md) |
| related | [to_token](/crates/oxide-library/src/library_file/mod/to_token.md) |
| related | [parse_token](/crates/oxide-library/src/library_file/mod/parse_token.md) |
| related | [to_token](/crates/oxide-library/src/library_file/mod/to_token.md) |
| related | [parse_token](/crates/oxide-library/src/library_file/mod/parse_token.md) |
| related | [ColumnTypeParseError](/crates/oxide-library/src/library_file/mod/ColumnTypeParseError.md) |
| related | [serialize](/crates/oxide-library/src/library_file/mod/serialize.md) |
| related | [serialize](/crates/oxide-library/src/library_file/mod/serialize.md) |
| related | [deserialize](/crates/oxide-library/src/library_file/mod/deserialize.md) |
| related | [deserialize](/crates/oxide-library/src/library_file/mod/deserialize.md) |
| related | [LibraryRow](/crates/oxide-library/src/library_file/mod/LibraryRow.md) |
| related | [cell](/crates/oxide-library/src/library_file/mod/cell.md) |
| related | [cell](/crates/oxide-library/src/library_file/mod/cell.md) |
| related | [LibraryFileError](/crates/oxide-library/src/library_file/mod/LibraryFileError.md) |
| related | [from](/crates/oxide-library/src/library_file/mod/from.md) |
| related | [from](/crates/oxide-library/src/library_file/mod/from.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
