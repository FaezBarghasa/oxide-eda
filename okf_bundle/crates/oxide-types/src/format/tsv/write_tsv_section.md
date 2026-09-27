---
okf_version: "0.2"
type: Function
title: write_tsv_section
description: "---------------------------------------------------------------------------"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/write_tsv_section
language: rust
---

# write_tsv_section

---------------------------------------------------------------------------

## Signature

```rust
pub(in crate::format) fn write_tsv_section(out: &mut String, name: &str, rows: &[R])
```

## Type Parameters

- `R: SnxTable`

## Visibility

- `pub(in crate::format)`

## Docstring

---------------------------------------------------------------------------
TSV section writer (TOML `[name]` + literal multi-line `content`)
---------------------------------------------------------------------------

## Source
Lines 371–377 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
| calls | [write_tsv_block](/crates/oxide-types/src/format/tsv/write_tsv_block.md) |
| calls | [escape_tsv_body_for_toml](/crates/oxide-types/src/format/tsv/escape_tsv_body_for_toml.md) |
| called_by | [write_string](/crates/oxide-types/src/format/mod/write_string.md) |
