---
okf_version: "0.2"
type: Function
title: parse_footprints_from_records
description: "Parse multiple [`Footprint`] primitives from a record slice."
resource: crates/oxide-altium-importer/src/pcblib_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T06:36:46Z"
concept_id: crates/oxide-altium-importer/src/pcblib_importer/parse_footprints_from_records
language: rust
---

# parse_footprints_from_records

Parse multiple [`Footprint`] primitives from a record slice.

## Signature

```rust
pub fn parse_footprints_from_records(records: &[AltiumRecord]) -> Result<Vec<Footprint>, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Parse multiple [`Footprint`] primitives from a record slice.

## Source
Lines 42–148 in `crates/oxide-altium-importer/src/pcblib_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcblib_importer](/crates/oxide-altium-importer/src/pcblib_importer.md) |
| called_by | [import_intlib_bytes](/crates/oxide-altium-importer/src/intlib_importer/import_intlib_bytes.md) |
| called_by | [import_pcblib_bytes](/crates/oxide-altium-importer/src/pcblib_importer/import_pcblib_bytes.md) |
| called_by | [test_pcblib_record_conversion_to_footprints](/crates/oxide-altium-importer/tests/importer_tests/test_pcblib_record_conversion_to_footprints.md) |
