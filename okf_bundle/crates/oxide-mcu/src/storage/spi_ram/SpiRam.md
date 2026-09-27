---
okf_version: "0.2"
type: Class
title: SpiRam
description: High-Speed SPI / Quad-SPI Pseudo-Static RAM (PSRAM).
resource: crates/oxide-mcu/src/storage/spi_ram.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:21Z"
concept_id: crates/oxide-mcu/src/storage/spi_ram/SpiRam
language: rust
---

# SpiRam

High-Speed SPI / Quad-SPI Pseudo-Static RAM (PSRAM).

## Signature

```rust
pub struct SpiRam
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

High-Speed SPI / Quad-SPI Pseudo-Static RAM (PSRAM).
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `size_bytes`
- `memory`
- `state`
- `cmd_buffer`
- `current_addr`

## Source
Lines 7–14 in `crates/oxide-mcu/src/storage/spi_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [spi_ram](/crates/oxide-mcu/src/storage/spi_ram.md) |
