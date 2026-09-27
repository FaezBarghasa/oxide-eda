---
okf_version: "0.2"
type: Class
title: I2cPeripheral
description: I2C Hardware peripheral model.
resource: crates/oxide-mcu/src/peripheral/i2c.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:51Z"
concept_id: crates/oxide-mcu/src/peripheral/i2c/I2cPeripheral
language: rust
---

# I2cPeripheral

I2C Hardware peripheral model.

## Signature

```rust
pub struct I2cPeripheral
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

I2C Hardware peripheral model.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `enabled`
- `own_address`
- `speed_mode`
- `addressing_mode`
- `clock_stretching`
- `bus_busy`
- `ack_received`

## Source
Lines 23–32 in `crates/oxide-mcu/src/peripheral/i2c.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [i2c](/crates/oxide-mcu/src/peripheral/i2c.md) |
