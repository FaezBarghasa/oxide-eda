---
okf_version: "0.2"
type: Class
title: VirtualPinState
description: State of a single virtual MCU pin.
resource: crates/oxide-mcu/src/pin_bridge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:51Z"
concept_id: crates/oxide-mcu/src/pin_bridge/VirtualPinState
language: rust
---

# VirtualPinState

State of a single virtual MCU pin.

## Signature

```rust
pub struct VirtualPinState
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

State of a single virtual MCU pin.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `pin_name`
- `function`
- `level`
- `analog_voltage`
- `pwm_duty_cycle`
- `connected_net`

## Source
Lines 70–77 in `crates/oxide-mcu/src/pin_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_bridge](/crates/oxide-mcu/src/pin_bridge.md) |
