---
okf_version: "0.2"
type: Function
title: pin_to_tsv_row
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/pin_to_tsv_row
language: rust
---

# pin_to_tsv_row

## Signature

```rust
fn pin_to_tsv_row(pin: &SymbolPin) -> Result<String, SymbolFileError>
```

## Source
Lines 164–197 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| calls | [pin_direction_token](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_direction_token.md) |
| calls | [fmt_f64](/crates/oxide-library/src/primitive/symbol/serde_tsv/fmt_f64.md) |
| calls | [pin_orientation_token](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_orientation_token.md) |
| calls | [fmt_opt_f64](/crates/oxide-library/src/primitive/symbol/serde_tsv/fmt_opt_f64.md) |
| calls | [pin_symbol_kind_token](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_symbol_kind_token.md) |
| called_by | [pins_to_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv/pins_to_tsv.md) |
