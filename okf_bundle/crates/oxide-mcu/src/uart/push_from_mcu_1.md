---
okf_version: "0.2"
type: Function
title: push_from_mcu
description: Receives a byte emitted by the virtual MCU UART transmitter.
resource: crates/oxide-mcu/src/uart.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:15Z"
concept_id: crates/oxide-mcu/src/uart/push_from_mcu_1
language: rust
---

# push_from_mcu

Receives a byte emitted by the virtual MCU UART transmitter.

## Signature

```rust
pub fn push_from_mcu(&mut self, byte: u8)
```

## Visibility

- `pub`

## Docstring

Receives a byte emitted by the virtual MCU UART transmitter.

## Source
Lines 28–36 in `crates/oxide-mcu/src/uart.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uart](/crates/oxide-mcu/src/uart.md) |
