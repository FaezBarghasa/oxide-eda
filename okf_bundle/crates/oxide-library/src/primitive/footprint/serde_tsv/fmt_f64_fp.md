---
okf_version: "0.2"
type: Function
title: fmt_f64_fp
description: "HI-10: see [`crate::primitive::symbol::fmt_f64`] — same NaN/inf guard."
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/fmt_f64_fp
language: rust
---

# fmt_f64_fp

HI-10: see [`crate::primitive::symbol::fmt_f64`] — same NaN/inf guard.

## Signature

```rust
fn fmt_f64_fp(v: f64) -> String
```

## Docstring

HI-10: see [`crate::primitive::symbol::fmt_f64`] — same NaN/inf guard.

## Source
Lines 36–45 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| called_by | [pad_to_tsv_row](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_to_tsv_row.md) |
