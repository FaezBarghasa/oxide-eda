---
okf_version: "0.2"
type: Function
title: pin_direction_convention
description: "Mirrors `crates/oxide-output/src/svg/symbols.rs`'s `pin_direction`"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_direction_convention
language: rust
---

# pin_direction_convention

Mirrors `crates/oxide-output/src/svg/symbols.rs`'s `pin_direction`

## Signature

```rust
fn pin_direction_convention(deg: f64) -> (f64, f64)
```

## Docstring

Mirrors `crates/oxide-output/src/svg/symbols.rs`'s `pin_direction`
exactly — same branches, same values — so this test fails the
instant either side's convention moves without the other.

## Source
Lines 158–170 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
| called_by | [pin_rotation_matches_oxide_output_pin_direction_convention](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_rotation_matches_oxide_output_pin_direction_convention.md) |
