---
okf_version: "0.2"
type: Function
title: format_f64
description: "Format an `f64` for TSV: trailing zeros stripped to keep diffs"
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/format_f64
language: rust
---

# format_f64

Format an `f64` for TSV: trailing zeros stripped to keep diffs

## Signature

```rust
pub(in crate::format) fn format_f64(f: f64) -> String
```

## Visibility

- `pub(in crate::format)`

## Docstring

Format an `f64` for TSV: trailing zeros stripped to keep diffs
minimal. Whole numbers emit as `0` rather than `0.0`.

HI-13: the previous `< 1e15` guard was looser than the actual
`i64::MAX as f64` boundary, and the `as i64` cast would wrap on
the gap between them. Use the real cast bound and route non-finite
inputs through `format!("{f}")` (which produces `"NaN"` / `"inf"`
— visible at parse time rather than silently corrupted).

## Source
Lines 357–365 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
