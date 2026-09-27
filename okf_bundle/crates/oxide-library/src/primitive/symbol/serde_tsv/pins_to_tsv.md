---
okf_version: "0.2"
type: Function
title: pins_to_tsv
description: "Encode a slice of pins as TSV — header row first, then one row"
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/pins_to_tsv
language: rust
---

# pins_to_tsv

Encode a slice of pins as TSV — header row first, then one row

## Signature

```rust
pub(crate) fn pins_to_tsv(pins: &[SymbolPin]) -> Result<String, SymbolFileError>
```

## Visibility

- `pub(crate)`

## Docstring

Encode a slice of pins as TSV — header row first, then one row
per pin. Empty slice still emits the header row so the round-trip
produces a parseable block.

## Source
Lines 202–211 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| calls | [pin_to_tsv_row](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_to_tsv_row.md) |
| called_by | [to_toml_string](/crates/oxide-library/src/primitive/symbol/mod/to_toml_string.md) |
| called_by | [pins_to_tsv_empty_emits_header_only](/crates/oxide-library/src/primitive/symbol/tests/pins_to_tsv_empty_emits_header_only.md) |
| called_by | [pins_to_tsv_rejects_newline_in_cell](/crates/oxide-library/src/primitive/symbol/tests/pins_to_tsv_rejects_newline_in_cell.md) |
| called_by | [pins_to_tsv_rejects_tab_in_cell](/crates/oxide-library/src/primitive/symbol/tests/pins_to_tsv_rejects_tab_in_cell.md) |
| called_by | [pins_to_tsv_rejects_triple_quote_in_cell](/crates/oxide-library/src/primitive/symbol/tests/pins_to_tsv_rejects_triple_quote_in_cell.md) |
