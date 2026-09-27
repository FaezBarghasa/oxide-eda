---
okf_version: "0.2"
type: Function
title: emit
description: Emit a BOM table as RFC 4180 CSV with UTF-8 BOM.
resource: crates/oxide-output/src/bom/csv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/bom/csv/emit
language: rust
---

# emit

Emit a BOM table as RFC 4180 CSV with UTF-8 BOM.

## Signature

```rust
pub fn emit(table: &BomTable, columns: &[BomColumn]) -> Result<Vec<u8>, BomError>
```

## Visibility

- `pub`

## Docstring

Emit a BOM table as RFC 4180 CSV with UTF-8 BOM.

## Source
Lines 8–42 in `crates/oxide-output/src/bom/csv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [csv](/crates/oxide-output/src/bom/csv.md) |
| calls | [write_csv_row](/crates/oxide-output/src/bom/csv/write_csv_row.md) |
