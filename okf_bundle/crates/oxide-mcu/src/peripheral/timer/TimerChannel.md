---
okf_version: "0.2"
type: Class
title: TimerChannel
description: A single Timer Capture/Compare Channel.
resource: crates/oxide-mcu/src/peripheral/timer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:11Z"
concept_id: crates/oxide-mcu/src/peripheral/timer/TimerChannel
language: rust
---

# TimerChannel

A single Timer Capture/Compare Channel.

## Signature

```rust
pub struct TimerChannel
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A single Timer Capture/Compare Channel.
[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]

## Methods

- `mode`
- `ccr`
- `duty_cycle`
- `output_state`
- `complementary_output`

## Source
Lines 27–33 in `crates/oxide-mcu/src/peripheral/timer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [timer](/crates/oxide-mcu/src/peripheral/timer.md) |
