---
okf_version: "0.2"
type: Function
title: set_gpio_output
description: Sets virtual GPIO level from MCU firmware.
resource: crates/oxide-mcu/src/pin_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:51Z"
concept_id: crates/oxide-mcu/src/pin_bridge/set_gpio_output
language: rust
---

# set_gpio_output

Sets virtual GPIO level from MCU firmware.

## Signature

```rust
impl PinBridge { pub fn set_gpio_output(&mut self, pin_name: &str, level: LogicLevel) }
```

## Visibility

- `pub`

## Docstring

Sets virtual GPIO level from MCU firmware.

## Source
Lines 117–122 in `crates/oxide-mcu/src/pin_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_bridge](/crates/oxide-mcu/src/pin_bridge.md) |
