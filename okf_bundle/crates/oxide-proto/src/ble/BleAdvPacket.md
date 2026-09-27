---
okf_version: "0.2"
type: Class
title: BleAdvPacket
description: Simulated BLE Advertising Payload.
resource: crates/oxide-proto/src/ble.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:53Z"
concept_id: crates/oxide-proto/src/ble/BleAdvPacket
language: rust
---

# BleAdvPacket

Simulated BLE Advertising Payload.

## Signature

```rust
pub struct BleAdvPacket
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Simulated BLE Advertising Payload.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `adv_type`
- `address`
- `device_name`
- `service_uuids`
- `rssi_dbm`
- `raw_data`

## Source
Lines 16–23 in `crates/oxide-proto/src/ble.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ble](/crates/oxide-proto/src/ble.md) |
