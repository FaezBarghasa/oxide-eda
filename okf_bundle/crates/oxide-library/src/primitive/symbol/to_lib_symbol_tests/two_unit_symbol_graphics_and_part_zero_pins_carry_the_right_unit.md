---
okf_version: "0.2"
type: Function
title: two_unit_symbol_graphics_and_part_zero_pins_carry_the_right_unit
description: "Acceptance criterion: a two-unit symbol whose part-1 and part-2"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/two_unit_symbol_graphics_and_part_zero_pins_carry_the_right_unit
language: rust
---

# two_unit_symbol_graphics_and_part_zero_pins_carry_the_right_unit

Acceptance criterion: a two-unit symbol whose part-1 and part-2

## Signature

```rust
fn two_unit_symbol_graphics_and_part_zero_pins_carry_the_right_unit()
```

## Decorators

- `test`

## Docstring

Acceptance criterion: a two-unit symbol whose part-1 and part-2
bodies differ converts to a `LibSymbol` whose graphics carry `unit =
1`/`unit = 2` respectively, whose shared (part 0) graphics carry
`unit = 0`, and whose Part Zero pins land on `unit = 0`.
[test]

## Source
Lines 43–81 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
