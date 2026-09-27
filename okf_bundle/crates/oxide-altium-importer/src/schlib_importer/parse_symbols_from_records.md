---
okf_version: "0.2"
type: Function
title: parse_symbols_from_records
description: "Parse multiple [`LibSymbol`]s from a record slice."
resource: crates/oxide-altium-importer/src/schlib_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T06:07:22Z"
concept_id: crates/oxide-altium-importer/src/schlib_importer/parse_symbols_from_records
language: rust
---

# parse_symbols_from_records

Parse multiple [`LibSymbol`]s from a record slice.

## Signature

```rust
pub fn parse_symbols_from_records(records: &[AltiumRecord]) -> Result<Vec<LibSymbol>, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Parse multiple [`LibSymbol`]s from a record slice.

## Source
Lines 47–202 in `crates/oxide-altium-importer/src/schlib_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schlib_importer](/crates/oxide-altium-importer/src/schlib_importer.md) |
| called_by | [import_intlib_bytes](/crates/oxide-altium-importer/src/intlib_importer/import_intlib_bytes.md) |
| called_by | [import_schlib_bytes](/crates/oxide-altium-importer/src/schlib_importer/import_schlib_bytes.md) |
| called_by | [test_schlib_record_conversion_to_lib_symbols](/crates/oxide-altium-importer/tests/importer_tests/test_schlib_record_conversion_to_lib_symbols.md) |
