---
okf_version: "0.2"
type: Function
title: read_u8
description: Read 8-bit byte.
resource: crates/oxide-mcu/src/storage/parallel_ram.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:29Z"
concept_id: crates/oxide-mcu/src/storage/parallel_ram/read_u8
language: rust
---

# read_u8

Read 8-bit byte.

## Signature

```rust
impl ParallelSram { pub fn read_u8(&self, addr: u32) -> Option<u8> }
```

## Visibility

- `pub`

## Docstring

Read 8-bit byte.

## Source
Lines 27–34 in `crates/oxide-mcu/src/storage/parallel_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_ram](/crates/oxide-mcu/src/storage/parallel_ram.md) |
