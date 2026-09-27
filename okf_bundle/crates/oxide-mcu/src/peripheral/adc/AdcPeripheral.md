---
okf_version: "0.2"
type: Class
title: AdcPeripheral
description: Multi-Channel SAR ADC Model.
resource: crates/oxide-mcu/src/peripheral/adc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:18Z"
concept_id: crates/oxide-mcu/src/peripheral/adc/AdcPeripheral
language: rust
---

# AdcPeripheral

Multi-Channel SAR ADC Model.

## Signature

```rust
pub struct AdcPeripheral
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Multi-Channel SAR ADC Model.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `enabled`
- `resolution`
- `conversion_mode`
- `vref_voltage`
- `sequence`
- `regular_data`
- `channels`
- `eoc_interrupt_enable`
- `watchdog_high_threshold`
- `watchdog_low_threshold`
- `watchdog_tripped`

## Source
Lines 43–56 in `crates/oxide-mcu/src/peripheral/adc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adc](/crates/oxide-mcu/src/peripheral/adc.md) |
