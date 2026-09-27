---
okf_version: "0.2"
type: Function
title: pin_shape_style
description: "`outside_edge_symbol` is the chosen authoritative glyph slot: its own"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_shape_style
language: rust
---

# pin_shape_style

`outside_edge_symbol` is the chosen authoritative glyph slot: its own

## Signature

```rust
fn pin_shape_style(outside_edge_symbol: PinSymbolKind) -> PinShapeStyle
```

## Docstring

`outside_edge_symbol` is the chosen authoritative glyph slot: its own
doc comment (`SymbolPin::outside_edge_symbol`) already calls it out as
the slot that "most commonly carries the inverted-pin dot" — the
modifier that matters electrically (bubble / clock edge). The other
three slots (`inside_symbol`, `inside_edge_symbol`, `outside_symbol`)
are dropped entirely; they only ever layer decorative glyphs the flat
`PinShapeStyle` has no room for anyway.

`PinSymbolKind` is `#[non_exhaustive]` for downstream crates, but this
match lives in the crate that defines it, so it stays exhaustive with
no wildcard arm — a future glyph fails this match at compile time
instead of silently degrading.

## Source
Lines 193–221 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol.md) |
| called_by | [lib_pin_from](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/lib_pin_from.md) |
