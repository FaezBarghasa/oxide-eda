---
okf_version: "0.2"
type: Function
title: set_pwm_channel
resource: crates/oxide-mcu/src/peripheral/timer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:11Z"
concept_id: crates/oxide-mcu/src/peripheral/timer/set_pwm_channel
language: rust
---

# set_pwm_channel

## Signature

```rust
impl TimerPeripheral { pub fn set_pwm_channel(&mut self, channel_idx: usize, ccr_val: u32) }
```

## Visibility

- `pub`

## Source
Lines 64–70 in `crates/oxide-mcu/src/peripheral/timer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [timer](/crates/oxide-mcu/src/peripheral/timer.md) |
