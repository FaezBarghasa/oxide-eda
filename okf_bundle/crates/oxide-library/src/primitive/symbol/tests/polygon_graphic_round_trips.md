---
okf_version: "0.2"
type: Function
title: polygon_graphic_round_trips
description: "A `Polygon` graphic's vertices, fill, and stroke width all survive"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/polygon_graphic_round_trips
language: rust
---

# polygon_graphic_round_trips

A `Polygon` graphic's vertices, fill, and stroke width all survive

## Signature

```rust
fn polygon_graphic_round_trips()
```

## Decorators

- `test`

## Docstring

A `Polygon` graphic's vertices, fill, and stroke width all survive
a `.snxsym` save/load round-trip. A new tagged VARIANT (unlike a
`#[serde(default)]` field addition) is backward-compat only — an
old build can't deserialize an externally-tagged `kind = "polygon"`
it doesn't know about — so a file containing one is written as
`SYMBOL_FILE_FORMAT_TOKEN_V2` ("snxsym/v2"), not v1.
[test]

## Source
Lines 517–543 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
