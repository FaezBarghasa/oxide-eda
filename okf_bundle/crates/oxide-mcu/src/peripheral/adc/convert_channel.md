---
okf_version: "0.2"
type: Function
title: convert_channel
description: Performs ADC conversion for the selected channel in sequence.
resource: crates/oxide-mcu/src/peripheral/adc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:18Z"
concept_id: crates/oxide-mcu/src/peripheral/adc/convert_channel
language: rust
---

# convert_channel

Performs ADC conversion for the selected channel in sequence.

## Signature

```rust
impl AdcPeripheral { pub fn convert_channel(&mut self, channel: u8) -> u32 }
```

## Visibility

- `pub`

## Docstring

Performs ADC conversion for the selected channel in sequence.
Returns converted integer count.

## Source
Lines 93–107 in `crates/oxide-mcu/src/peripheral/adc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adc](/crates/oxide-mcu/src/peripheral/adc.md) |
