---
okf_version: "0.2"
type: Class
title: EthernetMacPeripheral
description: 10/100M Ethernet MAC Hardware peripheral model.
resource: crates/oxide-mcu/src/peripheral/ethernet.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:08Z"
concept_id: crates/oxide-mcu/src/peripheral/ethernet/EthernetMacPeripheral
language: rust
---

# EthernetMacPeripheral

10/100M Ethernet MAC Hardware peripheral model.

## Signature

```rust
pub struct EthernetMacPeripheral
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

10/100M Ethernet MAC Hardware peripheral model.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `enabled`
- `mac_address`
- `tx_descriptors`
- `rx_descriptors`
- `link_up`

## Source
Lines 16–23 in `crates/oxide-mcu/src/peripheral/ethernet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ethernet](/crates/oxide-mcu/src/peripheral/ethernet.md) |
