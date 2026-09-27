---
okf_version: "0.2"
type: Function
title: write_csv_row
description: Write a single CSV row with proper quoting and escaping per RFC 4180.
resource: crates/oxide-output/src/bom/csv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/bom/csv/write_csv_row
language: rust
---

# write_csv_row

Write a single CSV row with proper quoting and escaping per RFC 4180.

## Signature

```rust
fn write_csv_row(output: &mut Vec<u8>, fields: &[impl AsRef<str>]) -> Result<(), BomError>
```

## Docstring

Write a single CSV row with proper quoting and escaping per RFC 4180.

## Source
Lines 45–71 in `crates/oxide-output/src/bom/csv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [csv](/crates/oxide-output/src/bom/csv.md) |
| called_by | [emit](/crates/oxide-output/src/bom/csv/emit.md) |
