---
okf_version: "0.2"
type: Function
title: import_schlib_bytes
description: "Import all symbols from an Altium `.SchLib` file byte slice."
resource: crates/oxide-altium-importer/src/schlib_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T06:07:22Z"
concept_id: crates/oxide-altium-importer/src/schlib_importer/import_schlib_bytes
language: rust
---

# import_schlib_bytes

Import all symbols from an Altium `.SchLib` file byte slice.

## Signature

```rust
pub fn import_schlib_bytes(bytes: &[u8]) -> Result<Vec<LibSymbol>, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Import all symbols from an Altium `.SchLib` file byte slice.

## Source
Lines 17–44 in `crates/oxide-altium-importer/src/schlib_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schlib_importer](/crates/oxide-altium-importer/src/schlib_importer.md) |
| calls | [parse_record_stream](/crates/oxide-altium-importer/src/record/parse_record_stream.md) |
| calls | [parse_symbols_from_records](/crates/oxide-altium-importer/src/schlib_importer/parse_symbols_from_records.md) |
| called_by | [import_altium_file](/crates/oxide-altium-importer/src/lib/import_altium_file.md) |
