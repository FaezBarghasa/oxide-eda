---
okf_version: "0.2"
type: Function
title: read_data
description: Read raw ADC integer output.
resource: crates/oxide-mcu/src/peripheral/adc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:18Z"
concept_id: crates/oxide-mcu/src/peripheral/adc/read_data
language: rust
---

# read_data

Read raw ADC integer output.

## Signature

```rust
impl AdcPeripheral { pub fn read_data(&mut self) -> u32 }
```

## Visibility

- `pub`

## Docstring

Read raw ADC integer output.

## Source
Lines 110–112 in `crates/oxide-mcu/src/peripheral/adc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adc](/crates/oxide-mcu/src/peripheral/adc.md) |
