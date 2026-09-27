---
okf_version: "0.2"
type: Function
title: read_byte_for_mcu
description: "Reads next byte if available for MCU firmware `USART_ReceiveData()`."
resource: crates/oxide-mcu/src/uart.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:15Z"
concept_id: crates/oxide-mcu/src/uart/read_byte_for_mcu
language: rust
---

# read_byte_for_mcu

Reads next byte if available for MCU firmware `USART_ReceiveData()`.

## Signature

```rust
impl VirtualUart { pub fn read_byte_for_mcu(&mut self) -> Option<u8> }
```

## Visibility

- `pub`

## Docstring

Reads next byte if available for MCU firmware `USART_ReceiveData()`.

## Source
Lines 46–48 in `crates/oxide-mcu/src/uart.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [uart](/crates/oxide-mcu/src/uart.md) |
