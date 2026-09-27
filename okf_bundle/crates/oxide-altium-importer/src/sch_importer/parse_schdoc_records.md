---
okf_version: "0.2"
type: Function
title: parse_schdoc_records
description: "Convert parsed Altium schematic records into an Oxide [`SchematicSheet`]."
resource: crates/oxide-altium-importer/src/sch_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T06:00:50Z"
concept_id: crates/oxide-altium-importer/src/sch_importer/parse_schdoc_records
language: rust
---

# parse_schdoc_records

Convert parsed Altium schematic records into an Oxide [`SchematicSheet`].

## Signature

```rust
pub fn parse_schdoc_records(records: &[AltiumRecord]) -> Result<SchematicSheet, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Convert parsed Altium schematic records into an Oxide [`SchematicSheet`].

## Source
Lines 24–262 in `crates/oxide-altium-importer/src/sch_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_importer](/crates/oxide-altium-importer/src/sch_importer.md) |
| called_by | [import_schdoc_bytes](/crates/oxide-altium-importer/src/sch_importer/import_schdoc_bytes.md) |
| called_by | [test_schdoc_record_conversion_to_schematic_sheet](/crates/oxide-altium-importer/tests/importer_tests/test_schdoc_record_conversion_to_schematic_sheet.md) |
