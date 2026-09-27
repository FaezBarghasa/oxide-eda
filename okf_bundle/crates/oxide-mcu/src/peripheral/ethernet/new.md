---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-mcu/src/peripheral/ethernet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:08Z"
concept_id: crates/oxide-mcu/src/peripheral/ethernet/new
language: rust
---

# new

## Signature

```rust
impl EthernetMacPeripheral { pub fn new(name: impl Into<String>, mac: [u8; 6]) -> Self }
```

## Visibility

- `pub`

## Source
Lines 26–35 in `crates/oxide-mcu/src/peripheral/ethernet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ethernet](/crates/oxide-mcu/src/peripheral/ethernet.md) |
