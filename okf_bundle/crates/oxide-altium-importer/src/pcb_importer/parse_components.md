---
okf_version: "0.2"
type: Function
title: parse_components
resource: crates/oxide-altium-importer/src/pcb_importer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:14:54Z"
concept_id: crates/oxide-altium-importer/src/pcb_importer/parse_components
language: rust
---

# parse_components

## Signature

```rust
fn parse_components(records: &[AltiumRecord], board: &mut PcbBoard)
```

## Source
Lines 155–181 in `crates/oxide-altium-importer/src/pcb_importer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_importer](/crates/oxide-altium-importer/src/pcb_importer.md) |
| called_by | [import_pcbdoc_bytes](/crates/oxide-altium-importer/src/pcb_importer/import_pcbdoc_bytes.md) |
