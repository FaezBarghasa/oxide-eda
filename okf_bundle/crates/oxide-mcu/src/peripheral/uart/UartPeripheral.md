---
okf_version: "0.2"
type: Class
title: UartPeripheral
description: Full UART/USART hardware peripheral model.
resource: crates/oxide-mcu/src/peripheral/uart.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:39Z"
concept_id: crates/oxide-mcu/src/peripheral/uart/UartPeripheral
language: rust
---

# UartPeripheral

Full UART/USART hardware peripheral model.

## Signature

```rust
pub struct UartPeripheral
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Full UART/USART hardware peripheral model.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `enabled`
- `tx_enabled`
- `rx_enabled`
- `baud_rate`
- `word_length`
- `parity`
- `stop_bits`
- `hw_flow_control`
- `rs485_driver_enable`
- `tx_fifo`
- `rx_fifo`
- `tx_empty`
- `rx_not_empty`

## Source
Lines 33–48 in `crates/oxide-mcu/src/peripheral/uart.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uart](/crates/oxide-mcu/src/peripheral/uart.md) |
