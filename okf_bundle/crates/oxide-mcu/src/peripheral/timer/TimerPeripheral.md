---
okf_version: "0.2"
type: Class
title: TimerPeripheral
description: "Hardware Timer & PWM peripheral model."
resource: crates/oxide-mcu/src/peripheral/timer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:11Z"
concept_id: crates/oxide-mcu/src/peripheral/timer/TimerPeripheral
language: rust
---

# TimerPeripheral

Hardware Timer & PWM peripheral model.

## Signature

```rust
pub struct TimerPeripheral
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Hardware Timer & PWM peripheral model.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `enabled`
- `counter_mode`
- `prescaler`
- `auto_reload`
- `counter`
- `repetition_counter`
- `dead_time_cycles`
- `channels`

## Source
Lines 37–47 in `crates/oxide-mcu/src/peripheral/timer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [timer](/crates/oxide-mcu/src/peripheral/timer.md) |
