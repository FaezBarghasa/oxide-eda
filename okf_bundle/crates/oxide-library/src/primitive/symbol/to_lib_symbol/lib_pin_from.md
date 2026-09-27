---
okf_version: "0.2"
type: Function
title: lib_pin_from
description: "`SymbolPin.part_number` (Altium-style: `0` = Part Zero, shared across"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol/lib_pin_from
language: rust
---

# lib_pin_from

`SymbolPin.part_number` (Altium-style: `0` = Part Zero, shared across

## Signature

```rust
fn lib_pin_from(pin: &SymbolPin) -> LibPin
```

## Docstring

`SymbolPin.part_number` (Altium-style: `0` = Part Zero, shared across
every unit) maps straight onto `LibPin.unit` — consumers already read
`unit == 0` as "common to all units"
(`crates/oxide-net/src/build/mod.rs`, `lp.unit != 0 && lp.unit !=
sym.unit`).

## Source
Lines 114–131 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol.md) |
| calls | [pin_direction](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_direction.md) |
| calls | [pin_shape_style](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_shape_style.md) |
| calls | [pin_rotation_deg](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_rotation_deg.md) |
