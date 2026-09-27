---
okf_version: "0.2"
type: Function
title: export_pcap
description: Generates standard libpcap binary file header and recorded packet stream.
resource: crates/oxide-proto/src/ethernet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-proto"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:28:39Z"
concept_id: crates/oxide-proto/src/ethernet/export_pcap
language: rust
---

# export_pcap

Generates standard libpcap binary file header and recorded packet stream.

## Signature

```rust
impl VirtualEthernetBus { pub fn export_pcap(&self) -> Vec<u8> }
```

## Visibility

- `pub`

## Docstring

Generates standard libpcap binary file header and recorded packet stream.

## Source
Lines 32–64 in `crates/oxide-proto/src/ethernet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ethernet](/crates/oxide-proto/src/ethernet.md) |
