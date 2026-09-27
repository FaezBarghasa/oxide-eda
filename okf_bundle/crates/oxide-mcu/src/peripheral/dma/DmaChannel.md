---
okf_version: "0.2"
type: Class
title: DmaChannel
description: A single DMA Stream / Channel.
resource: crates/oxide-mcu/src/peripheral/dma.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:57Z"
concept_id: crates/oxide-mcu/src/peripheral/dma/DmaChannel
language: rust
---

# DmaChannel

A single DMA Stream / Channel.

## Signature

```rust
pub struct DmaChannel
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A single DMA Stream / Channel.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `enabled`
- `direction`
- `priority`
- `circular_mode`
- `source_addr`
- `dest_addr`
- `total_items`
- `current_items_remaining`
- `half_transfer_interrupt`
- `transfer_complete_interrupt`
- `error_interrupt`
- `transfer_complete`
- `half_transfer_complete`

## Source
Lines 24–38 in `crates/oxide-mcu/src/peripheral/dma.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dma](/crates/oxide-mcu/src/peripheral/dma.md) |
