---
okf_version: "0.2"
type: Function
title: transfer_byte
description: Full-duplex SPI byte exchange.
resource: crates/oxide-mcu/src/storage/eeprom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:00Z"
concept_id: crates/oxide-mcu/src/storage/eeprom/transfer_byte_1
language: rust
---

# transfer_byte

Full-duplex SPI byte exchange.

## Signature

```rust
pub fn transfer_byte(&mut self, byte: u8) -> u8
```

## Visibility

- `pub`

## Docstring

Full-duplex SPI byte exchange.

## Source
Lines 138–194 in `crates/oxide-mcu/src/storage/eeprom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eeprom](/crates/oxide-mcu/src/storage/eeprom.md) |
