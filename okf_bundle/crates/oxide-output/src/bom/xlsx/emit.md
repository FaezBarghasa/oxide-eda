---
okf_version: "0.2"
type: Function
title: emit
description: Emit a BOM table as XLSX with styled header row and auto-fit widths.
resource: crates/oxide-output/src/bom/xlsx.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/bom/xlsx/emit
language: rust
---

# emit

Emit a BOM table as XLSX with styled header row and auto-fit widths.

## Signature

```rust
pub fn emit(table: &BomTable, columns: &[BomColumn]) -> Result<Vec<u8>, BomError>
```

## Visibility

- `pub`

## Docstring

Emit a BOM table as XLSX with styled header row and auto-fit widths.

## Source
Lines 20–89 in `crates/oxide-output/src/bom/xlsx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [xlsx](/crates/oxide-output/src/bom/xlsx.md) |
