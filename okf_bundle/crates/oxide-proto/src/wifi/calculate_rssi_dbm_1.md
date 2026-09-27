---
okf_version: "0.2"
type: Function
title: calculate_rssi_dbm
description: Calculates received signal strength (RSSI) based on free-space path loss (FSPL).
resource: crates/oxide-proto/src/wifi.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:29:34Z"
concept_id: crates/oxide-proto/src/wifi/calculate_rssi_dbm_1
language: rust
---

# calculate_rssi_dbm

Calculates received signal strength (RSSI) based on free-space path loss (FSPL).

## Signature

```rust
pub fn calculate_rssi_dbm(&self, distance_meters: f64) -> f64
```

## Visibility

- `pub`

## Docstring

Calculates received signal strength (RSSI) based on free-space path loss (FSPL).
$FSPL(dB) = 20 \log_{10}(d) + 20 \log_{10}(f) - 147.55$

## Source
Lines 52–57 in `crates/oxide-proto/src/wifi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wifi](/crates/oxide-proto/src/wifi.md) |
