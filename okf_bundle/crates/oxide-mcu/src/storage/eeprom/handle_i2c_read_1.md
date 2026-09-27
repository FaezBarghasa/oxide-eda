---
okf_version: "0.2"
type: Function
title: handle_i2c_read
description: Handles sequential I2C read from current internal address.
resource: crates/oxide-mcu/src/storage/eeprom.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:00Z"
concept_id: crates/oxide-mcu/src/storage/eeprom/handle_i2c_read_1
language: rust
---

# handle_i2c_read

Handles sequential I2C read from current internal address.

## Signature

```rust
pub fn handle_i2c_read(&mut self, len: usize) -> Vec<u8>
```

## Visibility

- `pub`

## Docstring

Handles sequential I2C read from current internal address.

## Source
Lines 81–89 in `crates/oxide-mcu/src/storage/eeprom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eeprom](/crates/oxide-mcu/src/storage/eeprom.md) |
