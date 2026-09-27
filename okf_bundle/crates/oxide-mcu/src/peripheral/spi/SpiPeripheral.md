---
okf_version: "0.2"
type: Class
title: SpiPeripheral
description: SPI Hardware peripheral model.
resource: crates/oxide-mcu/src/peripheral/spi.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:16:26Z"
concept_id: crates/oxide-mcu/src/peripheral/spi/SpiPeripheral
language: rust
---

# SpiPeripheral

SPI Hardware peripheral model.

## Signature

```rust
pub struct SpiPeripheral
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

SPI Hardware peripheral model.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `enabled`
- `role`
- `mode`
- `data_size`
- `baud_prescaler`
- `tx_fifo`
- `rx_fifo`

## Source
Lines 36–45 in `crates/oxide-mcu/src/peripheral/spi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [spi](/crates/oxide-mcu/src/peripheral/spi.md) |
