---
okf_version: "0.2"
type: Class
title: GpioPinState
description: State of a single GPIO pin inside a port.
resource: crates/oxide-mcu/src/peripheral/gpio.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:03Z"
concept_id: crates/oxide-mcu/src/peripheral/gpio/GpioPinState
language: rust
---

# GpioPinState

State of a single GPIO pin inside a port.

## Signature

```rust
pub struct GpioPinState
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

State of a single GPIO pin inside a port.
[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]

## Methods

- `mode`
- `otype`
- `speed`
- `pull`
- `output_data`
- `input_data`
- `voltage`

## Source
Lines 41–49 in `crates/oxide-mcu/src/peripheral/gpio.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gpio](/crates/oxide-mcu/src/peripheral/gpio.md) |
