---
okf_version: "0.2"
type: Function
title: split_row
description: "Split a TSV row on whitespace, honouring `\"` quoting so cells"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/split_row
language: rust
---

# split_row

Split a TSV row on whitespace, honouring `"` quoting so cells

## Signature

```rust
fn split_row(line: &str) -> Vec<String>
```

## Docstring

Split a TSV row on whitespace, honouring `"` quoting so cells
containing spaces are kept atomic.

## Source
Lines 137–175 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
| called_by | [parse_tsv_block](/crates/oxide-types/src/format/tsv/parse_tsv_block.md) |
