---
okf_version: "0.2"
type: Module
title: symbols
description: Symbol emitters — library body graphics and pins.
resource: crates/oxide-output/src/svg/symbols.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/symbols
language: rust
---

# symbols

Symbol emitters — library body graphics and pins.

## Docstring

Symbol emitters — library body graphics and pins.

Emits each placed symbol's `LibSymbol` body graphics and its pins
(stubs, names, numbers), plus the world-space transform and field
style helpers those emitters rely on.

Extracted verbatim from the SVG exporter (`svg/mod.rs`); pure code
motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [push_symbol_lib_graphics](/crates/oxide-output/src/svg/symbols/push_symbol_lib_graphics.md) |
| related | [push_symbol_pins](/crates/oxide-output/src/svg/symbols/push_symbol_pins.md) |
| related | [pin_direction](/crates/oxide-output/src/svg/symbols/pin_direction.md) |
| related | [symbol_world_point](/crates/oxide-output/src/svg/symbols/symbol_world_point.md) |
| related | [symbol_eval_variables](/crates/oxide-output/src/svg/symbols/symbol_eval_variables.md) |
| related | [field_effective_style](/crates/oxide-output/src/svg/symbols/field_effective_style.md) |
