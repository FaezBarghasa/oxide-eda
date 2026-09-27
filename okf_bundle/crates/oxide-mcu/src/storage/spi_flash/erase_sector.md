---
okf_version: "0.2"
type: Function
title: erase_sector
description: Erases 4KB Sector.
resource: crates/oxide-mcu/src/storage/spi_flash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:11Z"
concept_id: crates/oxide-mcu/src/storage/spi_flash/erase_sector
language: rust
---

# erase_sector

Erases 4KB Sector.

## Signature

```rust
impl SpiFlash { pub fn erase_sector(&mut self, sector_addr: usize) }
```

## Visibility

- `pub`

## Docstring

Erases 4KB Sector.

## Source
Lines 77–84 in `crates/oxide-mcu/src/storage/spi_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [spi_flash](/crates/oxide-mcu/src/storage/spi_flash.md) |
