---
okf_version: "0.2"
type: Function
title: pad_shape_to_token
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/pad_shape_to_token
language: rust
---

# pad_shape_to_token

## Signature

```rust
pub(super) fn pad_shape_to_token(shape: &PadShape) -> Result<String, FootprintFileError>
```

## Visibility

- `pub(super)`

## Source
Lines 51–80 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| called_by | [pad_to_tsv_row](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_to_tsv_row.md) |
| called_by | [pad_shape_token_round_trip_each_variant](/crates/oxide-library/src/primitive/footprint/tests/pad_shape_token_round_trip_each_variant.md) |
