---
okf_version: "0.2"
type: Function
title: read_idr
description: Read Input Data Register (IDR) as 16-bit integer.
resource: crates/oxide-mcu/src/peripheral/gpio.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:03Z"
concept_id: crates/oxide-mcu/src/peripheral/gpio/read_idr
language: rust
---

# read_idr

Read Input Data Register (IDR) as 16-bit integer.

## Signature

```rust
impl GpioPort { pub fn read_idr(&self) -> u16 }
```

## Visibility

- `pub`

## Docstring

Read Input Data Register (IDR) as 16-bit integer.

## Source
Lines 104–112 in `crates/oxide-mcu/src/peripheral/gpio.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gpio](/crates/oxide-mcu/src/peripheral/gpio.md) |
