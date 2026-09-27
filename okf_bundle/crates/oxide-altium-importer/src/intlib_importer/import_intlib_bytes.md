---
okf_version: "0.2"
type: Function
title: import_intlib_bytes
description: Import an Altium Integrated Library (.IntLib) byte slice.
resource: crates/oxide-altium-importer/src/intlib_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T06:02:08Z"
concept_id: crates/oxide-altium-importer/src/intlib_importer/import_intlib_bytes
language: rust
---

# import_intlib_bytes

Import an Altium Integrated Library (.IntLib) byte slice.

## Signature

```rust
pub fn import_intlib_bytes(bytes: &[u8]) -> Result<ExtractedIntLib, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Import an Altium Integrated Library (.IntLib) byte slice.

## Source
Lines 20–47 in `crates/oxide-altium-importer/src/intlib_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [intlib_importer](/crates/oxide-altium-importer/src/intlib_importer.md) |
| calls | [parse_record_stream](/crates/oxide-altium-importer/src/record/parse_record_stream.md) |
| calls | [parse_symbols_from_records](/crates/oxide-altium-importer/src/schlib_importer/parse_symbols_from_records.md) |
| calls | [parse_footprints_from_records](/crates/oxide-altium-importer/src/pcblib_importer/parse_footprints_from_records.md) |
| called_by | [import_altium_file](/crates/oxide-altium-importer/src/lib/import_altium_file.md) |
