---
okf_version: "0.2"
type: Function
title: parse_f64_cell
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/parse_f64_cell
language: rust
---

# parse_f64_cell

## Signature

```rust
fn parse_f64_cell(col: &'static str, s: &str) -> Result<f64, SymbolFileError>
```

## Source
Lines 138–143 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| called_by | [parse_opt_f64_cell](/crates/oxide-library/src/primitive/symbol/serde_tsv/parse_opt_f64_cell.md) |
| called_by | [pin_from_tsv_row](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_from_tsv_row.md) |
