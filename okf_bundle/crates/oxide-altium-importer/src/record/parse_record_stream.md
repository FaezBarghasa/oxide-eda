---
okf_version: "0.2"
type: Function
title: parse_record_stream
description: Parse an entire stream of length-prefixed pipe-delimited records.
resource: crates/oxide-altium-importer/src/record.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:45:59Z"
concept_id: crates/oxide-altium-importer/src/record/parse_record_stream
language: rust
---

# parse_record_stream

Parse an entire stream of length-prefixed pipe-delimited records.

## Signature

```rust
pub fn parse_record_stream(data: &[u8]) -> Result<Vec<AltiumRecord>, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Parse an entire stream of length-prefixed pipe-delimited records.

## Source
Lines 61–104 in `crates/oxide-altium-importer/src/record.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [record](/crates/oxide-altium-importer/src/record.md) |
| called_by | [import_intlib_bytes](/crates/oxide-altium-importer/src/intlib_importer/import_intlib_bytes.md) |
| called_by | [import_pcbdoc_bytes](/crates/oxide-altium-importer/src/pcb_importer/import_pcbdoc_bytes.md) |
| called_by | [import_pcblib_bytes](/crates/oxide-altium-importer/src/pcblib_importer/import_pcblib_bytes.md) |
| called_by | [import_schdoc_bytes](/crates/oxide-altium-importer/src/sch_importer/import_schdoc_bytes.md) |
| called_by | [import_schlib_bytes](/crates/oxide-altium-importer/src/schlib_importer/import_schlib_bytes.md) |
| called_by | [test_length_prefixed_record_stream_parsing](/crates/oxide-altium-importer/tests/importer_tests/test_length_prefixed_record_stream_parsing.md) |
