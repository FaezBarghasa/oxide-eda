---
okf_version: "0.2"
type: Function
title: inject_rx_byte
description: Injects byte received from physical bus / terminal into RX FIFO.
resource: crates/oxide-mcu/src/peripheral/uart.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:39Z"
concept_id: crates/oxide-mcu/src/peripheral/uart/inject_rx_byte_1
language: rust
---

# inject_rx_byte

Injects byte received from physical bus / terminal into RX FIFO.

## Signature

```rust
pub fn inject_rx_byte(&mut self, byte: u8)
```

## Visibility

- `pub`

## Docstring

Injects byte received from physical bus / terminal into RX FIFO.

## Source
Lines 86–91 in `crates/oxide-mcu/src/peripheral/uart.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uart](/crates/oxide-mcu/src/peripheral/uart.md) |
