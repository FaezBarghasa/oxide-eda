---
okf_version: "0.2"
type: Function
title: parse_tsv_block
description: "Parse a TSV block: validate the header against `R::columns()`,"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/parse_tsv_block
language: rust
---

# parse_tsv_block

Parse a TSV block: validate the header against `R::columns()`,

## Signature

```rust
pub fn parse_tsv_block(block: &str, content: &str) -> Result<Vec<R>, FormatError>
```

## Type Parameters

- `R: SnxTable`

## Visibility

- `pub`

## Docstring

Parse a TSV block: validate the header against `R::columns()`,
then parse each data row through `R::from_row`.

## Source
Lines 236–277 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
| calls | [split_row](/crates/oxide-types/src/format/tsv/split_row.md) |
| calls | [decode_cell](/crates/oxide-types/src/format/tsv/decode_cell.md) |
| called_by | [tsv_writer_pads_columns_for_legibility](/crates/oxide-types/src/format/tests/tsv_writer_pads_columns_for_legibility.md) |
