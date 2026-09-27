---
okf_version: "0.2"
type: Function
title: graphic_fill_round_trips
description: "A graphic's fill colour survives a `.snxsym` save/load round-trip,"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/graphic_fill_round_trips
language: rust
---

# graphic_fill_round_trips

A graphic's fill colour survives a `.snxsym` save/load round-trip,

## Signature

```rust
fn graphic_fill_round_trips()
```

## Decorators

- `test`

## Docstring

A graphic's fill colour survives a `.snxsym` save/load round-trip,
proving the new `SymbolGraphic.fill` field serialises through the
TOML manifest alongside the rest of the graphic. Back-compat for
legacy files missing the field is covered by the additive
`#[serde(default)]`, same as `graphic_missing_part_number_defaults_to_zero`.
[test]

## Source
Lines 470–485 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
