---
okf_version: "0.2"
type: Function
title: import_altium_file
description: Import an Altium Designer file from a local filesystem path.
resource: crates/oxide-altium-importer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:55:23Z"
concept_id: crates/oxide-altium-importer/src/lib/import_altium_file
language: rust
---

# import_altium_file

Import an Altium Designer file from a local filesystem path.

## Signature

```rust
pub fn import_altium_file(path: P) -> Result<AltiumImportResult, AltiumImportError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Import an Altium Designer file from a local filesystem path.

## Source
Lines 37–71 in `crates/oxide-altium-importer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-altium-importer/src/lib.md) |
| calls | [import_schdoc_bytes](/crates/oxide-altium-importer/src/sch_importer/import_schdoc_bytes.md) |
| calls | [import_pcbdoc_bytes](/crates/oxide-altium-importer/src/pcb_importer/import_pcbdoc_bytes.md) |
| calls | [import_schlib_bytes](/crates/oxide-altium-importer/src/schlib_importer/import_schlib_bytes.md) |
| calls | [import_pcblib_bytes](/crates/oxide-altium-importer/src/pcblib_importer/import_pcblib_bytes.md) |
| calls | [import_intlib_bytes](/crates/oxide-altium-importer/src/intlib_importer/import_intlib_bytes.md) |
