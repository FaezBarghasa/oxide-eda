---
okf_version: "0.2"
type: Class
title: EthernetFrame
description: Captured Ethernet Packet.
resource: crates/oxide-proto/src/ethernet.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:39Z"
concept_id: crates/oxide-proto/src/ethernet/EthernetFrame
language: rust
---

# EthernetFrame

Captured Ethernet Packet.

## Signature

```rust
pub struct EthernetFrame
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Captured Ethernet Packet.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `timestamp_us`
- `src_mac`
- `dst_mac`
- `ether_type`
- `payload`

## Source
Lines 7–13 in `crates/oxide-proto/src/ethernet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ethernet](/crates/oxide-proto/src/ethernet.md) |
