---
okf_version: "0.2"
type: Function
title: parse_tsv
description: "Parse TSV text into a [`LibraryTable`]. The first non-empty line is"
resource: crates/oxide-library/src/library_file/codec.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/codec/parse_tsv
language: rust
---

# parse_tsv

Parse TSV text into a [`LibraryTable`]. The first non-empty line is

## Signature

```rust
fn parse_tsv(table_name: &str, tsv: &str) -> Result<LibraryTable, LibraryFileError>
```

## Docstring

Parse TSV text into a [`LibraryTable`]. The first non-empty line is
the header; subsequent lines are rows.

## Source
Lines 156–215 in `crates/oxide-library/src/library_file/codec.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codec](/crates/oxide-library/src/library_file/codec.md) |
| called_by | [parse](/crates/oxide-library/src/library_file/codec/parse.md) |
