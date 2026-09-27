---
okf_version: "0.2"
type: Function
title: write_u16
description: "Write 16-bit half-word with Byte High/Low Enable (`BHE`/`BLE`)."
resource: crates/oxide-mcu/src/storage/parallel_ram.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:29Z"
concept_id: crates/oxide-mcu/src/storage/parallel_ram/write_u16
language: rust
---

# write_u16

Write 16-bit half-word with Byte High/Low Enable (`BHE`/`BLE`).

## Signature

```rust
impl ParallelSram { pub fn write_u16(&mut self, addr: u32, val: u16, ble: bool, bhe: bool) -> bool }
```

## Visibility

- `pub`

## Docstring

Write 16-bit half-word with Byte High/Low Enable (`BHE`/`BLE`).

## Source
Lines 73–87 in `crates/oxide-mcu/src/storage/parallel_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_ram](/crates/oxide-mcu/src/storage/parallel_ram.md) |
