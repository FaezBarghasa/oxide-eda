---
okf_version: "0.2"
type: Class
title: VirtualWifiAp
description: Virtual Wi-Fi Access Point (AP) state.
resource: crates/oxide-proto/src/wifi.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:29:34Z"
concept_id: crates/oxide-proto/src/wifi/VirtualWifiAp
language: rust
---

# VirtualWifiAp

Virtual Wi-Fi Access Point (AP) state.

## Signature

```rust
pub struct VirtualWifiAp
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Virtual Wi-Fi Access Point (AP) state.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `ssid`
- `bssid`
- `channel`
- `tx_power_dbm`
- `connected_stations`

## Source
Lines 31–37 in `crates/oxide-proto/src/wifi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wifi](/crates/oxide-proto/src/wifi.md) |
