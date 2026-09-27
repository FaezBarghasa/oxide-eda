---
okf_version: "0.2"
type: Function
title: to_voltage
resource: crates/oxide-mcu/src/pin_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:51Z"
concept_id: crates/oxide-mcu/src/pin_bridge/to_voltage
language: rust
---

# to_voltage

## Signature

```rust
impl LogicLevel { pub fn to_voltage(&self, vdd: f64) -> f64 }
```

## Visibility

- `pub`

## Source
Lines 16–22 in `crates/oxide-mcu/src/pin_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_bridge](/crates/oxide-mcu/src/pin_bridge.md) |
