---
okf_version: "0.2"
type: Function
title: read_adc_raw_12bit
description: Converts analog voltage into 12-bit ADC raw integer count (0..4095).
resource: crates/oxide-mcu/src/pin_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:51Z"
concept_id: crates/oxide-mcu/src/pin_bridge/read_adc_raw_12bit
language: rust
---

# read_adc_raw_12bit

Converts analog voltage into 12-bit ADC raw integer count (0..4095).

## Signature

```rust
impl PinBridge { pub fn read_adc_raw_12bit(&self, pin_name: &str) -> Option<u16> }
```

## Visibility

- `pub`

## Docstring

Converts analog voltage into 12-bit ADC raw integer count (0..4095).

## Source
Lines 133–137 in `crates/oxide-mcu/src/pin_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_bridge](/crates/oxide-mcu/src/pin_bridge.md) |
