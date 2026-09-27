---
okf_version: "0.2"
type: Function
title: read_byte
description: Reads received byte into MCU firmware.
resource: crates/oxide-mcu/src/peripheral/uart.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:39Z"
concept_id: crates/oxide-mcu/src/peripheral/uart/read_byte
language: rust
---

# read_byte

Reads received byte into MCU firmware.

## Signature

```rust
impl UartPeripheral { pub fn read_byte(&mut self) -> Option<u8> }
```

## Visibility

- `pub`

## Docstring

Reads received byte into MCU firmware.

## Source
Lines 79–83 in `crates/oxide-mcu/src/peripheral/uart.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uart](/crates/oxide-mcu/src/peripheral/uart.md) |
