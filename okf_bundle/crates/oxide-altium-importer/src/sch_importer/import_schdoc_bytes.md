---
okf_version: "0.2"
type: Function
title: import_schdoc_bytes
description: "Import an Altium `.SchDoc` file from raw binary bytes."
resource: crates/oxide-altium-importer/src/sch_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T06:00:50Z"
concept_id: crates/oxide-altium-importer/src/sch_importer/import_schdoc_bytes
language: rust
---

# import_schdoc_bytes

Import an Altium `.SchDoc` file from raw binary bytes.

## Signature

```rust
pub fn import_schdoc_bytes(bytes: &[u8]) -> Result<SchematicSheet, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Import an Altium `.SchDoc` file from raw binary bytes.

## Source
Lines 16–21 in `crates/oxide-altium-importer/src/sch_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_importer](/crates/oxide-altium-importer/src/sch_importer.md) |
| calls | [parse_record_stream](/crates/oxide-altium-importer/src/record/parse_record_stream.md) |
| calls | [parse_schdoc_records](/crates/oxide-altium-importer/src/sch_importer/parse_schdoc_records.md) |
| called_by | [import_altium_file](/crates/oxide-altium-importer/src/lib/import_altium_file.md) |
