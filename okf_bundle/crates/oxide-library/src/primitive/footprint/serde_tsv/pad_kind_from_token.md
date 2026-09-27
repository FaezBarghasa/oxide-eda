---
okf_version: "0.2"
type: Function
title: pad_kind_from_token
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/pad_kind_from_token
language: rust
---

# pad_kind_from_token

## Signature

```rust
pub(super) fn pad_kind_from_token(s: &str) -> Result<PadKind, FootprintFileError>
```

## Visibility

- `pub(super)`

## Source
Lines 18–33 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
| called_by | [pad_from_tsv_row](/crates/oxide-library/src/primitive/footprint/serde_tsv/pad_from_tsv_row.md) |
| called_by | [pad_kind_token_round_trip_all_variants](/crates/oxide-library/src/primitive/footprint/tests/pad_kind_token_round_trip_all_variants.md) |
