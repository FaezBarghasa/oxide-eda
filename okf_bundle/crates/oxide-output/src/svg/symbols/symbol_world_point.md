---
okf_version: "0.2"
type: Function
title: symbol_world_point
resource: crates/oxide-output/src/svg/symbols.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/symbols/symbol_world_point
language: rust
---

# symbol_world_point

## Signature

```rust
fn symbol_world_point(sym: &Symbol, local: &Point) -> (f64, f64)
```

## Source
Lines 417–428 in `crates/oxide-output/src/svg/symbols.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbols](/crates/oxide-output/src/svg/symbols.md) |
| called_by | [push_symbol_lib_graphics](/crates/oxide-output/src/svg/symbols/push_symbol_lib_graphics.md) |
| called_by | [push_symbol_pins](/crates/oxide-output/src/svg/symbols/push_symbol_pins.md) |
