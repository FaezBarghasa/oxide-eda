---
okf_version: "0.2"
type: Function
title: to_lib_symbol
description: "Convert this library `Symbol` into a schematic `LibSymbol`."
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol/to_lib_symbol
language: rust
---

# to_lib_symbol

Convert this library `Symbol` into a schematic `LibSymbol`.

## Signature

```rust
impl Symbol { pub fn to_lib_symbol(&self, id: impl Into<String>) -> LibSymbol }
```

## Visibility

- `pub`

## Docstring

Convert this library `Symbol` into a schematic `LibSymbol`.

`id` is caller-supplied (e.g. a library-relative lookup key) —
this function does not invent an id scheme; that is part 2's job.

Header fields: `designator -> reference`, `comment -> value`,
`description -> description`. Fields `LibSymbol` has that `Symbol`
carries no equivalent for (`footprint`, `datasheet`, `keywords`,
`fp_filters`, the `in_bom`/`on_board`/`in_pos_files` visibility
flags, `duplicate_pin_numbers_are_jumpers`, the pin-number/name
display toggles, `pin_name_offset`) take `LibSymbol`'s own
defaults — this Symbol type simply has no source data for them.

## Source
Lines 86–106 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol.md) |
