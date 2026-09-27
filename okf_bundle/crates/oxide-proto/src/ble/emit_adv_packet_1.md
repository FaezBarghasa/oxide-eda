---
okf_version: "0.2"
type: Function
title: emit_adv_packet
description: Emits GAP advertising payload packet.
resource: crates/oxide-proto/src/ble.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:53Z"
concept_id: crates/oxide-proto/src/ble/emit_adv_packet_1
language: rust
---

# emit_adv_packet

Emits GAP advertising payload packet.

## Signature

```rust
pub fn emit_adv_packet(&self, rssi_dbm: i8) -> BleAdvPacket
```

## Visibility

- `pub`

## Docstring

Emits GAP advertising payload packet.

## Source
Lines 64–74 in `crates/oxide-proto/src/ble.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ble](/crates/oxide-proto/src/ble.md) |
