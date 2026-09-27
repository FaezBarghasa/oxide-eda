---
okf_version: "0.2"
type: Function
title: pin_direction_token
description: "---- Pin TSV codec --------------------------------------------------"
resource: crates/oxide-library/src/primitive/symbol/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/serde_tsv/pin_direction_token
language: rust
---

# pin_direction_token

---- Pin TSV codec --------------------------------------------------

## Signature

```rust
pub(super) fn pin_direction_token(d: PinDirection) -> &'static str
```

## Visibility

- `pub(super)`

## Docstring

---- Pin TSV codec --------------------------------------------------

## Source
Lines 7–20 in `crates/oxide-library/src/primitive/symbol/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv.md) |
| called_by | [pin_to_tsv_row](/crates/oxide-library/src/primitive/symbol/serde_tsv/pin_to_tsv_row.md) |
| called_by | [pin_direction_token_round_trip_all_variants](/crates/oxide-library/src/primitive/symbol/tests/pin_direction_token_round_trip_all_variants.md) |
