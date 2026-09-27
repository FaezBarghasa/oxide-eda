---
okf_version: "0.2"
type: Function
title: transfer_byte
description: Full-duplex byte transfer on Master.
resource: crates/oxide-mcu/src/peripheral/spi.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:16:26Z"
concept_id: crates/oxide-mcu/src/peripheral/spi/transfer_byte
language: rust
---

# transfer_byte

Full-duplex byte transfer on Master.

## Signature

```rust
impl SpiPeripheral { pub fn transfer_byte(&mut self, tx_byte: u8) -> u8 }
```

## Visibility

- `pub`

## Docstring

Full-duplex byte transfer on Master.

## Source
Lines 62–68 in `crates/oxide-mcu/src/peripheral/spi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [spi](/crates/oxide-mcu/src/peripheral/spi.md) |
