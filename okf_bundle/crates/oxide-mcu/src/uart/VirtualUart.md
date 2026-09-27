---
okf_version: "0.2"
type: Class
title: VirtualUart
description: Virtual Serial Port FIFO buffer connecting MCU USART/UART to the EDA Console.
resource: crates/oxide-mcu/src/uart.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:15Z"
concept_id: crates/oxide-mcu/src/uart/VirtualUart
language: rust
---

# VirtualUart

Virtual Serial Port FIFO buffer connecting MCU USART/UART to the EDA Console.

## Signature

```rust
pub struct VirtualUart
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Virtual Serial Port FIFO buffer connecting MCU USART/UART to the EDA Console.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `baud_rate`
- `tx_buffer`
- `rx_buffer`
- `history_lines`
- `current_line`

## Source
Lines 8–14 in `crates/oxide-mcu/src/uart.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uart](/crates/oxide-mcu/src/uart.md) |
