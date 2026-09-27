---
okf_version: "0.2"
type: Function
title: point_at_deg
description: "Point on a circle at angle `deg`, `center + radius * (cos, sin)` —"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol/point_at_deg
language: rust
---

# point_at_deg

Point on a circle at angle `deg`, `center + radius * (cos, sin)` —

## Signature

```rust
fn point_at_deg(center: [f64; 2], radius: f64, deg: f64) -> Point
```

## Docstring

Point on a circle at angle `deg`, `center + radius * (cos, sin)` —
the same "standard math convention, no axis flips" this crate's own
`chain.rs` documents and relies on for `SymbolGraphicKind::Arc`.

## Source
Lines 250–256 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol.md) |
| called_by | [arc_points](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/arc_points.md) |
