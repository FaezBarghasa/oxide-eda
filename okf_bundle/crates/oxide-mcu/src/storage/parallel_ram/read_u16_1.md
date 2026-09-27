---
okf_version: "0.2"
type: Function
title: read_u16
description: Read 16-bit half-word.
resource: crates/oxide-mcu/src/storage/parallel_ram.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:45:29Z"
concept_id: crates/oxide-mcu/src/storage/parallel_ram/read_u16_1
language: rust
---

# read_u16

Read 16-bit half-word.

## Signature

```rust
pub fn read_u16(&self, addr: u32) -> Option<u16>
```

## Visibility

- `pub`

## Docstring

Read 16-bit half-word.

## Source
Lines 37–44 in `crates/oxide-mcu/src/storage/parallel_ram.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_ram](/crates/oxide-mcu/src/storage/parallel_ram.md) |
