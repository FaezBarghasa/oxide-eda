---
okf_version: "0.2"
type: Class
title: UsbEndpoint
description: "[derive(Debug, Clone, Default, Serialize, Deserialize)]"
resource: crates/oxide-mcu/src/peripheral/usb.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:14Z"
concept_id: crates/oxide-mcu/src/peripheral/usb/UsbEndpoint
language: rust
---

# UsbEndpoint

[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Signature

```rust
pub struct UsbEndpoint
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `number`
- `ep_type`
- `max_packet_size`
- `tx_buffer`
- `rx_buffer`
- `stalled`

## Source
Lines 15–22 in `crates/oxide-mcu/src/peripheral/usb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [usb](/crates/oxide-mcu/src/peripheral/usb.md) |
