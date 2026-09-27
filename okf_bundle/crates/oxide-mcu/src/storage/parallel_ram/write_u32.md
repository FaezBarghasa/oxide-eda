---
okf_version: "0.2"
type: Function
title: write_u32
description: Write 32-bit word.
resource: crates/oxide-mcu/src/storage/parallel_ram.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:29Z"
concept_id: crates/oxide-mcu/src/storage/parallel_ram/write_u32
language: rust
---

# write_u32

Write 32-bit word.

## Signature

```rust
impl ParallelSram { pub fn write_u32(&mut self, addr: u32, val: u32) -> bool }
```

## Visibility

- `pub`

## Docstring

Write 32-bit word.

## Source
Lines 90–99 in `crates/oxide-mcu/src/storage/parallel_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_ram](/crates/oxide-mcu/src/storage/parallel_ram.md) |
