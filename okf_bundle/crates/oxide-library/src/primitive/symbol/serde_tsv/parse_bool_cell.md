---
okf_version: "0.2"
type: Function
title: parse_bool_cell
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/parse_bool_cell
language: rust
---

# parse_bool_cell

## Signature

```rust
fn parse_bool_cell(col: &'static str, s: &str) -> Result<bool, SymbolFileError>
```

## Source
Lines 153–162 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| called_by | [pin_from_tsv_row](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_from_tsv_row.md) |
