---
okf_version: "0.2"
type: Function
title: write_u8
description: Write 8-bit byte.
resource: crates/oxide-mcu/src/storage/parallel_ram.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:29Z"
concept_id: crates/oxide-mcu/src/storage/parallel_ram/write_u8
language: rust
---

# write_u8

Write 8-bit byte.

## Signature

```rust
impl ParallelSram { pub fn write_u8(&mut self, addr: u32, val: u8) -> bool }
```

## Visibility

- `pub`

## Docstring

Write 8-bit byte.

## Source
Lines 62–70 in `crates/oxide-mcu/src/storage/parallel_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_ram](/crates/oxide-mcu/src/storage/parallel_ram.md) |
