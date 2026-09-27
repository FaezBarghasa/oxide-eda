---
okf_version: "0.2"
type: Class
title: WifiFrame
description: Simulated 802.11 Wi-Fi Frame.
resource: crates/oxide-proto/src/wifi.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:29:34Z"
concept_id: crates/oxide-proto/src/wifi/WifiFrame
language: rust
---

# WifiFrame

Simulated 802.11 Wi-Fi Frame.

## Signature

```rust
pub struct WifiFrame
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Simulated 802.11 Wi-Fi Frame.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `kind`
- `ssid`
- `bssid`
- `client_mac`
- `channel`
- `rssi_dbm`
- `payload`

## Source
Lines 19–27 in `crates/oxide-proto/src/wifi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wifi](/crates/oxide-proto/src/wifi.md) |
