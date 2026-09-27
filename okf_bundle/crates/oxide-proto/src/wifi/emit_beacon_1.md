---
okf_version: "0.2"
type: Function
title: emit_beacon
description: Emits 802.11 Beacon frame.
resource: crates/oxide-proto/src/wifi.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:29:34Z"
concept_id: crates/oxide-proto/src/wifi/emit_beacon_1
language: rust
---

# emit_beacon

Emits 802.11 Beacon frame.

## Signature

```rust
pub fn emit_beacon(&self, distance_meters: f64) -> WifiFrame
```

## Visibility

- `pub`

## Docstring

Emits 802.11 Beacon frame.

## Source
Lines 60–70 in `crates/oxide-proto/src/wifi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wifi](/crates/oxide-proto/src/wifi.md) |
