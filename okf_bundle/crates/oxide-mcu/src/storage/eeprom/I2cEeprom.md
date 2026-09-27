---
okf_version: "0.2"
type: Class
title: I2cEeprom
description: "I2C EEPROM Model (e.g. Microchip 24LC04, 24LC64, 24LC256, 24LC512, 24LC1025)."
resource: crates/oxide-mcu/src/storage/eeprom.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:00Z"
concept_id: crates/oxide-mcu/src/storage/eeprom/I2cEeprom
language: rust
---

# I2cEeprom

I2C EEPROM Model (e.g. Microchip 24LC04, 24LC64, 24LC256, 24LC512, 24LC1025).

## Signature

```rust
pub struct I2cEeprom
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

I2C EEPROM Model (e.g. Microchip 24LC04, 24LC64, 24LC256, 24LC512, 24LC1025).
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `i2c_address`
- `size_bytes`
- `page_size_bytes`
- `memory`
- `write_protected`
- `internal_address`
- `is_16bit_addr`

## Source
Lines 7–16 in `crates/oxide-mcu/src/storage/eeprom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eeprom](/crates/oxide-mcu/src/storage/eeprom.md) |
