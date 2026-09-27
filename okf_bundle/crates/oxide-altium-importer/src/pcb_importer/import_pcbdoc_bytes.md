---
okf_version: "0.2"
type: Function
title: import_pcbdoc_bytes
description: "Import an Altium `.PcbDoc` file from raw binary bytes."
resource: crates/oxide-altium-importer/src/pcb_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:14:54Z"
concept_id: crates/oxide-altium-importer/src/pcb_importer/import_pcbdoc_bytes
language: rust
---

# import_pcbdoc_bytes

Import an Altium `.PcbDoc` file from raw binary bytes.

## Signature

```rust
pub fn import_pcbdoc_bytes(bytes: &[u8]) -> Result<PcbBoard, AltiumImportError>
```

## Visibility

- `pub`

## Docstring

Import an Altium `.PcbDoc` file from raw binary bytes.

## Source
Lines 10–96 in `crates/oxide-altium-importer/src/pcb_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_importer](/crates/oxide-altium-importer/src/pcb_importer.md) |
| calls | [parse_record_stream](/crates/oxide-altium-importer/src/record/parse_record_stream.md) |
| calls | [parse_board_metadata](/crates/oxide-altium-importer/src/pcb_importer/parse_board_metadata.md) |
| calls | [parse_tracks](/crates/oxide-altium-importer/src/pcb_importer/parse_tracks.md) |
| calls | [parse_vias](/crates/oxide-altium-importer/src/pcb_importer/parse_vias.md) |
| calls | [parse_components](/crates/oxide-altium-importer/src/pcb_importer/parse_components.md) |
| calls | [parse_all_pcb_records](/crates/oxide-altium-importer/src/pcb_importer/parse_all_pcb_records.md) |
| called_by | [import_altium_file](/crates/oxide-altium-importer/src/lib/import_altium_file.md) |
