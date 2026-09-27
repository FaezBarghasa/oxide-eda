---
okf_version: "0.2"
type: Function
title: pin_from_tsv_row
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/pin_from_tsv_row
language: rust
---

# pin_from_tsv_row

## Signature

```rust
fn pin_from_tsv_row(cells: &[&str]) -> Result<SymbolPin, SymbolFileError>
```

## Source
Lines 249–286 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| calls | [pin_direction_from_token](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_direction_from_token.md) |
| calls | [parse_f64_cell](/crates/oxide-library/src/primitive/symbol/serde_tsv/parse_f64_cell.md) |
| calls | [pin_orientation_from_token](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_orientation_from_token.md) |
| calls | [parse_opt_f64_cell](/crates/oxide-library/src/primitive/symbol/serde_tsv/parse_opt_f64_cell.md) |
| calls | [parse_bool_cell](/crates/oxide-library/src/primitive/symbol/serde_tsv/parse_bool_cell.md) |
| calls | [pin_symbol_kind_from_token](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_symbol_kind_from_token.md) |
| called_by | [pins_from_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv/pins_from_tsv.md) |
