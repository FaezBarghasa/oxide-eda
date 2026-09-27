---
okf_version: "0.2"
type: Function
title: update_from_spice_voltage
description: Updates pin voltage sampled from the SPICE analog engine.
resource: crates/oxide-mcu/src/pin_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:51Z"
concept_id: crates/oxide-mcu/src/pin_bridge/update_from_spice_voltage
language: rust
---

# update_from_spice_voltage

Updates pin voltage sampled from the SPICE analog engine.

## Signature

```rust
impl PinBridge { pub fn update_from_spice_voltage(&mut self, pin_name: &str, voltage: f64) }
```

## Visibility

- `pub`

## Docstring

Updates pin voltage sampled from the SPICE analog engine.

## Source
Lines 125–130 in `crates/oxide-mcu/src/pin_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_bridge](/crates/oxide-mcu/src/pin_bridge.md) |
