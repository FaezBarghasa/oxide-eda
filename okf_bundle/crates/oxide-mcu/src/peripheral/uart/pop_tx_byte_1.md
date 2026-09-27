---
okf_version: "0.2"
type: Function
title: pop_tx_byte
description: Pops byte from TX FIFO to transmit over physical bus.
resource: crates/oxide-mcu/src/peripheral/uart.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:39Z"
concept_id: crates/oxide-mcu/src/peripheral/uart/pop_tx_byte_1
language: rust
---

# pop_tx_byte

Pops byte from TX FIFO to transmit over physical bus.

## Signature

```rust
pub fn pop_tx_byte(&mut self) -> Option<u8>
```

## Visibility

- `pub`

## Docstring

Pops byte from TX FIFO to transmit over physical bus.

## Source
Lines 94–98 in `crates/oxide-mcu/src/peripheral/uart.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uart](/crates/oxide-mcu/src/peripheral/uart.md) |
