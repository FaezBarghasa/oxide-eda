---
okf_version: "0.2"
type: Function
title: transmit_frame
description: Broadcasts or unicasts an Ethernet frame across connected simulated devices.
resource: crates/oxide-proto/src/ethernet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:39Z"
concept_id: crates/oxide-proto/src/ethernet/transmit_frame_1
language: rust
---

# transmit_frame

Broadcasts or unicasts an Ethernet frame across connected simulated devices.

## Signature

```rust
pub fn transmit_frame(&mut self, frame: EthernetFrame)
```

## Visibility

- `pub`

## Docstring

Broadcasts or unicasts an Ethernet frame across connected simulated devices.

## Source
Lines 27–29 in `crates/oxide-proto/src/ethernet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ethernet](/crates/oxide-proto/src/ethernet.md) |
