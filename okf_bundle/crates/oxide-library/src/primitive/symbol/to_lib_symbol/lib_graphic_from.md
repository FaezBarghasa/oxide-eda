---
okf_version: "0.2"
type: Function
title: lib_graphic_from
description: "`SymbolGraphic.part_number` maps straight onto `LibGraphic.unit` — see"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol/lib_graphic_from
language: rust
---

# lib_graphic_from

`SymbolGraphic.part_number` maps straight onto `LibGraphic.unit` — see

## Signature

```rust
fn lib_graphic_from(graphic: &SymbolGraphic) -> LibGraphic
```

## Docstring

`SymbolGraphic.part_number` maps straight onto `LibGraphic.unit` — see
[`lib_pin_from`]'s doc comment for the shared `unit == 0` = "common to
every unit" contract.

## Source
Lines 226–232 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol.md) |
| calls | [to_graphic](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/to_graphic.md) |
