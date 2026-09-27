---
okf_version: "0.2"
type: Class
title: LibraryTable
description: "Parsed in-memory view of one `[tables.<name>]` block."
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/LibraryTable
language: rust
---

# LibraryTable

Parsed in-memory view of one `[tables.<name>]` block.

## Signature

```rust
pub struct LibraryTable
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq)`

## Visibility

- `pub`

## Docstring

Parsed in-memory view of one `[tables.<name>]` block.

`columns` is the TSV header row, in declaration order (preserved
across round-trip so column reordering is a deliberate user action).
Each [`LibraryRow`] stores its values keyed by column name; the
shared [`LibraryTable::columns`] is the schema.

`column_types` is the optional typed-sidecar map: per-column type
declarations that drive UI sort behaviour (numeric sort vs lexical),
validation on edit, and rendering hints (checkboxes for bool, drop-
downs for enum). Columns without an entry default to
[`ColumnType::String`] so untyped tables keep working unchanged.
[derive(Debug, Clone, Default, PartialEq)]

## Methods

- `columns`
- `rows`
- `column_types`

## Source
Lines 112–116 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
