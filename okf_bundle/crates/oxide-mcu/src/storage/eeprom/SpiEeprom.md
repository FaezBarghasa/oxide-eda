---
okf_version: "0.2"
type: Class
title: SpiEeprom
description: "SPI EEPROM Model (e.g. Microchip 25LC040, 25LC640, 25LC256, 25LC1024)."
resource: crates/oxide-mcu/src/storage/eeprom.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:00Z"
concept_id: crates/oxide-mcu/src/storage/eeprom/SpiEeprom
language: rust
---

# SpiEeprom

SPI EEPROM Model (e.g. Microchip 25LC040, 25LC640, 25LC256, 25LC1024).

## Signature

```rust
pub struct SpiEeprom
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

SPI EEPROM Model (e.g. Microchip 25LC040, 25LC640, 25LC256, 25LC1024).
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `size_bytes`
- `page_size_bytes`
- `memory`
- `write_enable_latch`
- `status_register`
- `state`
- `cmd_buffer`
- `read_address`
- `write_address`

## Source
Lines 94–105 in `crates/oxide-mcu/src/storage/eeprom.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eeprom](/crates/oxide-mcu/src/storage/eeprom.md) |
