---
okf_version: "0.2"
type: Function
title: parse_f64_cell_fp
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/parse_f64_cell_fp
language: rust
---

# parse_f64_cell_fp

## Signature

```rust
fn parse_f64_cell_fp(col: &'static str, s: &str) -> Result<f64, FootprintFileError>
```

## Source
Lines 168–174 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| called_by | [pad_from_tsv_row](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_from_tsv_row.md) |
| called_by | [parse_opt_f64_cell_fp](/crates/oxide-library/src/primitive/footprint/serde_tsv/parse_opt_f64_cell_fp.md) |
