---
okf_version: "0.2"
type: Function
title: read_u32
description: Read 32-bit word.
resource: crates/oxide-mcu/src/storage/parallel_ram.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:29Z"
concept_id: crates/oxide-mcu/src/storage/parallel_ram/read_u32
language: rust
---

# read_u32

Read 32-bit word.

## Signature

```rust
impl ParallelSram { pub fn read_u32(&self, addr: u32) -> Option<u32> }
```

## Visibility

- `pub`

## Docstring

Read 32-bit word.

## Source
Lines 47–59 in `crates/oxide-mcu/src/storage/parallel_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_ram](/crates/oxide-mcu/src/storage/parallel_ram.md) |
