---
okf_version: "0.2"
type: Function
title: write_tsv_block
description: "Write a TSV block: header row + one row per item, columns aligned"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/write_tsv_block
language: rust
---

# write_tsv_block

Write a TSV block: header row + one row per item, columns aligned

## Signature

```rust
pub fn write_tsv_block(rows: &[R]) -> String
```

## Type Parameters

- `R: SnxTable`

## Visibility

- `pub`

## Docstring

Write a TSV block: header row + one row per item, columns aligned
to the longest cell per column for legibility (matches the
`.snxlib` writer's whitespace-flexible style).

## Source
Lines 180–232 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
| calls | [encode_cell](/crates/oxide-types/src/format/tsv/encode_cell.md) |
| called_by | [tsv_writer_pads_columns_for_legibility](/crates/oxide-types/src/format/tests/tsv_writer_pads_columns_for_legibility.md) |
| called_by | [write_tsv_section](/crates/oxide-types/src/format/tsv/write_tsv_section.md) |
