---
okf_version: "0.2"
type: Function
title: handle_i2c_write
description: Handles incoming I2C write transaction (address + data).
resource: crates/oxide-mcu/src/storage/eeprom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:00Z"
concept_id: crates/oxide-mcu/src/storage/eeprom/handle_i2c_write
language: rust
---

# handle_i2c_write

Handles incoming I2C write transaction (address + data).

## Signature

```rust
impl I2cEeprom { pub fn handle_i2c_write(&mut self, data: &[u8]) -> bool }
```

## Visibility

- `pub`

## Docstring

Handles incoming I2C write transaction (address + data).

## Source
Lines 46–78 in `crates/oxide-mcu/src/storage/eeprom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eeprom](/crates/oxide-mcu/src/storage/eeprom.md) |
