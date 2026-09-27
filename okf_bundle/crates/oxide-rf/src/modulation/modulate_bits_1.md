---
okf_version: "0.2"
type: Function
title: modulate_bits
description: Maps bit sequence into I/Q baseband symbols according to modulation scheme.
resource: crates/oxide-rf/src/modulation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:26:41Z"
concept_id: crates/oxide-rf/src/modulation/modulate_bits_1
language: rust
---

# modulate_bits

Maps bit sequence into I/Q baseband symbols according to modulation scheme.

## Signature

```rust
pub fn modulate_bits(bits: &[bool], scheme: ModulationScheme) -> Vec<IqSymbol>
```

## Visibility

- `pub`

## Docstring

Maps bit sequence into I/Q baseband symbols according to modulation scheme.

## Source
Lines 54–114 in `crates/oxide-rf/src/modulation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modulation](/crates/oxide-rf/src/modulation.md) |
