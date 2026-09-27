---
okf_version: "0.2"
type: Function
title: write_bsrr
description: Atomic Bit Set / Reset Register (BSRR) write emulation.
resource: crates/oxide-mcu/src/peripheral/gpio.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:03Z"
concept_id: crates/oxide-mcu/src/peripheral/gpio/write_bsrr
language: rust
---

# write_bsrr

Atomic Bit Set / Reset Register (BSRR) write emulation.

## Signature

```rust
impl GpioPort { pub fn write_bsrr(&mut self, bsrr_val: u32) }
```

## Visibility

- `pub`

## Docstring

Atomic Bit Set / Reset Register (BSRR) write emulation.
Bits 0..15 set the pin High, bits 16..31 reset the pin Low.

## Source
Lines 85–101 in `crates/oxide-mcu/src/peripheral/gpio.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gpio](/crates/oxide-mcu/src/peripheral/gpio.md) |
