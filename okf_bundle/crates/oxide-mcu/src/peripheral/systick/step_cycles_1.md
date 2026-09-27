---
okf_version: "0.2"
type: Function
title: step_cycles
description: Advances the timer by N core clock cycles.
resource: crates/oxide-mcu/src/peripheral/systick.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:36:56Z"
concept_id: crates/oxide-mcu/src/peripheral/systick/step_cycles_1
language: rust
---

# step_cycles

Advances the timer by N core clock cycles.

## Signature

```rust
pub fn step_cycles(&mut self, cycles: u32) -> bool
```

## Visibility

- `pub`

## Docstring

Advances the timer by N core clock cycles.
Returns `true` if a SysTick exception / interrupt should be triggered.

## Source
Lines 30–48 in `crates/oxide-mcu/src/peripheral/systick.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [systick](/crates/oxide-mcu/src/peripheral/systick.md) |
