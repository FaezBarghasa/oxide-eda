---
okf_version: "0.2"
type: Class
title: MemoryModel
description: Memory Architecture Model.
resource: crates/oxide-mcu/src/arch/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:40:05Z"
concept_id: crates/oxide-mcu/src/arch/mod/MemoryModel
language: rust
---

# MemoryModel

Memory Architecture Model.

## Signature

```rust
pub enum MemoryModel
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Memory Architecture Model.
[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `flash_word_size_bytes`
- `sram_size_bytes`
- `eeprom_size_bytes`

## Source
Lines 44–53 in `crates/oxide-mcu/src/arch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arch](/crates/oxide-mcu/src/arch/mod.md) |
