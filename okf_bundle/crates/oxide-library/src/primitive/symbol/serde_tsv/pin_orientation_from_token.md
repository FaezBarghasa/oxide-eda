---
okf_version: "0.2"
type: Function
title: pin_orientation_from_token
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/pin_orientation_from_token
language: rust
---

# pin_orientation_from_token

## Signature

```rust
pub(super) fn pin_orientation_from_token(s: &str) -> Result<PinOrientation, SymbolFileError>
```

## Visibility

- `pub(super)`

## Source
Lines 52–65 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| called_by | [pin_from_tsv_row](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_from_tsv_row.md) |
| called_by | [pin_orientation_token_round_trip_all_variants](/crates/oxide-library/src/primitive/symbol/tests/pin_orientation_token_round_trip_all_variants.md) |
