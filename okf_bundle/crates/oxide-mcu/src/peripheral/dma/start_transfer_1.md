---
okf_version: "0.2"
type: Function
title: start_transfer
resource: crates/oxide-mcu/src/peripheral/dma.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:57Z"
concept_id: crates/oxide-mcu/src/peripheral/dma/start_transfer_1
language: rust
---

# start_transfer

## Signature

```rust
pub fn start_transfer(&mut self, channel_idx: usize, src: u32, dst: u32, count: usize)
```

## Visibility

- `pub`

## Source
Lines 55–65 in `crates/oxide-mcu/src/peripheral/dma.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dma](/crates/oxide-mcu/src/peripheral/dma.md) |
