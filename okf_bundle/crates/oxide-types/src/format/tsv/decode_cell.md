---
okf_version: "0.2"
type: Function
title: decode_cell
description: "Decode a single TSV cell. `\"\"` and a bare `-` return empty (the"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/decode_cell
language: rust
---

# decode_cell

Decode a single TSV cell. `""` and a bare `-` return empty (the

## Signature

```rust
pub(in crate::format) fn decode_cell(cell: &str) -> String
```

## Visibility

- `pub(in crate::format)`

## Docstring

Decode a single TSV cell. `""` and a bare `-` return empty (the
latter per the format spec); surrounding double quotes strip, with
inner `""` collapsing back to `"` and `\\` / `\n` / `\r` / `\t`
reversing the escapes `encode_cell` writes.

## Source
Lines 55–91 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
| called_by | [encode_decode_cell_round_trips_dangerous_characters](/crates/oxide-types/src/format/tests/encode_decode_cell_round_trips_dangerous_characters.md) |
| called_by | [parse_tsv_block](/crates/oxide-types/src/format/tsv/parse_tsv_block.md) |
