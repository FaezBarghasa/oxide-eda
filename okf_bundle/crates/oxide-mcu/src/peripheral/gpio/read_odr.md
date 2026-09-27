---
okf_version: "0.2"
type: Function
title: read_odr
description: Read Output Data Register (ODR) as 16-bit integer.
resource: crates/oxide-mcu/src/peripheral/gpio.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:03Z"
concept_id: crates/oxide-mcu/src/peripheral/gpio/read_odr
language: rust
---

# read_odr

Read Output Data Register (ODR) as 16-bit integer.

## Signature

```rust
impl GpioPort { pub fn read_odr(&self) -> u16 }
```

## Visibility

- `pub`

## Docstring

Read Output Data Register (ODR) as 16-bit integer.

## Source
Lines 115–123 in `crates/oxide-mcu/src/peripheral/gpio.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gpio](/crates/oxide-mcu/src/peripheral/gpio.md) |
