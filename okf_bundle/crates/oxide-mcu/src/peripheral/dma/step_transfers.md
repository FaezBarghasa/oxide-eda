---
okf_version: "0.2"
type: Function
title: step_transfers
description: Steps DMA transfers by transferring N items.
resource: crates/oxide-mcu/src/peripheral/dma.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:57Z"
concept_id: crates/oxide-mcu/src/peripheral/dma/step_transfers
language: rust
---

# step_transfers

Steps DMA transfers by transferring N items.

## Signature

```rust
impl DmaController { pub fn step_transfers(&mut self, items_to_transfer: usize) }
```

## Visibility

- `pub`

## Docstring

Steps DMA transfers by transferring N items.

## Source
Lines 68–91 in `crates/oxide-mcu/src/peripheral/dma.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dma](/crates/oxide-mcu/src/peripheral/dma.md) |
