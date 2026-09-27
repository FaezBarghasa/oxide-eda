---
okf_version: "0.2"
type: Class
title: SpiFlash
description: SPI / QSPI NOR Flash Emulation.
resource: crates/oxide-mcu/src/storage/spi_flash.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:11Z"
concept_id: crates/oxide-mcu/src/storage/spi_flash/SpiFlash
language: rust
---

# SpiFlash

SPI / QSPI NOR Flash Emulation.

## Signature

```rust
pub struct SpiFlash
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

SPI / QSPI NOR Flash Emulation.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `size_bytes`
- `page_size_bytes`
- `sector_size_bytes`
- `block_size_bytes`
- `memory`
- `write_enable_latch`
- `status_register_1`
- `status_register_2`
- `state`
- `cmd_buffer`
- `current_addr`

## Source
Lines 7–20 in `crates/oxide-mcu/src/storage/spi_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [spi_flash](/crates/oxide-mcu/src/storage/spi_flash.md) |
