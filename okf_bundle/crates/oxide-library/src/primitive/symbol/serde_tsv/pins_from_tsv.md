---
okf_version: "0.2"
type: Function
title: pins_from_tsv
description: "Parse a `pins_tsv` payload back into `Vec<SymbolPin>`. The first"
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/pins_from_tsv
language: rust
---

# pins_from_tsv

Parse a `pins_tsv` payload back into `Vec<SymbolPin>`. The first

## Signature

```rust
pub(crate) fn pins_from_tsv(tsv: &str) -> Result<Vec<SymbolPin>, SymbolFileError>
```

## Visibility

- `pub(crate)`

## Docstring

Parse a `pins_tsv` payload back into `Vec<SymbolPin>`. The first
non-empty line is the header and must equal [`PIN_TSV_COLUMNS`];
each subsequent line is a pin row.

## Source
Lines 216–247 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| calls | [pin_from_tsv_row](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_from_tsv_row.md) |
| called_by | [from_toml_str](/crates/oxide-library/src/primitive/symbol/mod/from_toml_str.md) |
| called_by | [pins_from_tsv_rejects_cell_count_mismatch](/crates/oxide-library/src/primitive/symbol/tests/pins_from_tsv_rejects_cell_count_mismatch.md) |
| called_by | [pins_from_tsv_rejects_schema_mismatch](/crates/oxide-library/src/primitive/symbol/tests/pins_from_tsv_rejects_schema_mismatch.md) |
