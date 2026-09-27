---
okf_version: "0.2"
type: Function
title: read_adc_raw_16bit
description: Converts analog voltage into 16-bit ADC raw integer count (0..65535).
resource: crates/oxide-mcu/src/pin_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:51Z"
concept_id: crates/oxide-mcu/src/pin_bridge/read_adc_raw_16bit
language: rust
---

# read_adc_raw_16bit

Converts analog voltage into 16-bit ADC raw integer count (0..65535).

## Signature

```rust
impl PinBridge { pub fn read_adc_raw_16bit(&self, pin_name: &str) -> Option<u16> }
```

## Visibility

- `pub`

## Docstring

Converts analog voltage into 16-bit ADC raw integer count (0..65535).

## Source
Lines 140–144 in `crates/oxide-mcu/src/pin_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin_bridge](/crates/oxide-mcu/src/pin_bridge.md) |
