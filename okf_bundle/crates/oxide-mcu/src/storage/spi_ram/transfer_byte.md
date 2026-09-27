---
okf_version: "0.2"
type: Function
title: transfer_byte
description: Full-duplex SPI byte exchange.
resource: crates/oxide-mcu/src/storage/spi_ram.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:21Z"
concept_id: crates/oxide-mcu/src/storage/spi_ram/transfer_byte
language: rust
---

# transfer_byte

Full-duplex SPI byte exchange.

## Signature

```rust
impl SpiRam { pub fn transfer_byte(&mut self, byte: u8) -> u8 }
```

## Visibility

- `pub`

## Docstring

Full-duplex SPI byte exchange.

## Source
Lines 55–126 in `crates/oxide-mcu/src/storage/spi_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [spi_ram](/crates/oxide-mcu/src/storage/spi_ram.md) |
