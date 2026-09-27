---
okf_version: "0.2"
type: Function
title: erase_chip
description: Full chip erase (0xC7 / 0x60).
resource: crates/oxide-mcu/src/storage/spi_flash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:11Z"
concept_id: crates/oxide-mcu/src/storage/spi_flash/erase_chip
language: rust
---

# erase_chip

Full chip erase (0xC7 / 0x60).

## Signature

```rust
impl SpiFlash { pub fn erase_chip(&mut self) }
```

## Visibility

- `pub`

## Docstring

Full chip erase (0xC7 / 0x60).

## Source
Lines 97–101 in `crates/oxide-mcu/src/storage/spi_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [spi_flash](/crates/oxide-mcu/src/storage/spi_flash.md) |
