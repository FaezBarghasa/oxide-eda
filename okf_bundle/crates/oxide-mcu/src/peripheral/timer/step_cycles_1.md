---
okf_version: "0.2"
type: Function
title: step_cycles
description: Advances timer counter by clock cycles.
resource: crates/oxide-mcu/src/peripheral/timer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:11Z"
concept_id: crates/oxide-mcu/src/peripheral/timer/step_cycles_1
language: rust
---

# step_cycles

Advances timer counter by clock cycles.

## Signature

```rust
pub fn step_cycles(&mut self, cycles: u32) -> bool
```

## Visibility

- `pub`

## Docstring

Advances timer counter by clock cycles.
Returns `true` if an update event (overflow/underflow interrupt) occurred.

## Source
Lines 83–130 in `crates/oxide-mcu/src/peripheral/timer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [timer](/crates/oxide-mcu/src/peripheral/timer.md) |
