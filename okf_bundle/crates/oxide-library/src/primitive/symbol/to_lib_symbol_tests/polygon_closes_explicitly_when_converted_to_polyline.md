---
okf_version: "0.2"
type: Function
title: polygon_closes_explicitly_when_converted_to_polyline
description: "`SymbolGraphicKind::Polygon`'s vertex ring is closed implicitly; the"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/polygon_closes_explicitly_when_converted_to_polyline
language: rust
---

# polygon_closes_explicitly_when_converted_to_polyline

`SymbolGraphicKind::Polygon`'s vertex ring is closed implicitly; the

## Signature

```rust
fn polygon_closes_explicitly_when_converted_to_polyline()
```

## Decorators

- `test`

## Docstring

`SymbolGraphicKind::Polygon`'s vertex ring is closed implicitly; the
converted `Graphic::Polyline` closes it explicitly by repeating the
first vertex.
[test]

## Source
Lines 350–369 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
