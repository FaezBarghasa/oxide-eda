---
okf_version: "0.2"
type: Function
title: import_pcblib_bytes
description: "Import all footprints from an Altium `.PcbLib` file byte slice."
resource: crates/oxide-altium-importer/src/pcblib_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T06:36:46Z"
concept_id: crates/oxide-altium-importer/src/pcblib_importer/import_pcblib_bytes
language: rust
---

# import_pcblib_bytes

Import all footprints from an Altium `.PcbLib` file byte slice.

## Signature

```rust
pub fn import_pcblib_bytes(bytes: &[u8]) -> Result<Vec<Footprint>, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Import all footprints from an Altium `.PcbLib` file byte slice.

## Source
Lines 12–39 in `crates/oxide-altium-importer/src/pcblib_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcblib_importer](/crates/oxide-altium-importer/src/pcblib_importer.md) |
| calls | [parse_record_stream](/crates/oxide-altium-importer/src/record/parse_record_stream.md) |
| calls | [parse_footprints_from_records](/crates/oxide-altium-importer/src/pcblib_importer/parse_footprints_from_records.md) |
| called_by | [import_altium_file](/crates/oxide-altium-importer/src/lib/import_altium_file.md) |
