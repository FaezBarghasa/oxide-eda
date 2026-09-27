---
okf_version: "0.2"
type: Function
title: pin_direction
description: "Total, panic-free map from the library's 10-variant [`PinDirection`]"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_direction
language: rust
---

# pin_direction

Total, panic-free map from the library's 10-variant [`PinDirection`]

## Signature

```rust
fn pin_direction(source: PinDirection) -> SchematicPinDirection
```

## Docstring

Total, panic-free map from the library's 10-variant [`PinDirection`]
to the schematic's independently-curated 14-variant
`oxide_types::schematic::PinDirection`. Every source variant is
pinned by a test in [`super::to_lib_symbol_tests`].

`PinDirection` is `#[non_exhaustive]` for downstream crates, but this
match lives in the crate that defines it, so it stays exhaustive with
no wildcard arm — a future variant fails this match at compile time
instead of silently defaulting.

## Source
Lines 158–179 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol.md) |
| called_by | [lib_pin_from](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/lib_pin_from.md) |
