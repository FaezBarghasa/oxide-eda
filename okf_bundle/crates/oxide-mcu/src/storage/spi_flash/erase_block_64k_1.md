---
okf_version: "0.2"
type: Function
title: erase_block_64k
description: Erases 64KB Block.
resource: crates/oxide-mcu/src/storage/spi_flash.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:11Z"
concept_id: crates/oxide-mcu/src/storage/spi_flash/erase_block_64k_1
language: rust
---

# erase_block_64k

Erases 64KB Block.

## Signature

```rust
pub fn erase_block_64k(&mut self, block_addr: usize)
```

## Visibility

- `pub`

## Docstring

Erases 64KB Block.

## Source
Lines 87–94 in `crates/oxide-mcu/src/storage/spi_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [spi_flash](/crates/oxide-mcu/src/storage/spi_flash.md) |
