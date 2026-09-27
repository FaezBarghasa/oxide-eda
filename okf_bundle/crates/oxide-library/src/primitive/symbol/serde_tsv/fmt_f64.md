---
okf_version: "0.2"
type: Function
title: fmt_f64
description: "Format an `f64` for a TSV cell. `0.0` emits literally as `\"0\"` so"
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/fmt_f64
language: rust
---

# fmt_f64

Format an `f64` for a TSV cell. `0.0` emits literally as `"0"` so

## Signature

```rust
fn fmt_f64(v: f64) -> String
```

## Docstring

Format an `f64` for a TSV cell. `0.0` emits literally as `"0"` so
the most common default is short; non-zero values use the
shortest precision-preserving form via `Display`. Cells must be
re-parseable by `f64::from_str`.

HI-10: non-finite values (NaN / ±Inf) silently become `"NaN"` /
`"inf"` strings via `Display`, which fail to re-parse. Surface as
an empty cell — round-trip lands on `parse_f64_cell`'s "invalid
numeric" error rather than corrupting the file.

## Source
Lines 122–132 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| called_by | [pin_to_tsv_row](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_to_tsv_row.md) |
