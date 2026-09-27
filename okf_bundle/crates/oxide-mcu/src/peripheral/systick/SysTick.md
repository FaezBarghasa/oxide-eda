---
okf_version: "0.2"
type: Class
title: SysTick
description: ARM SysTick 24-bit core system timer.
resource: crates/oxide-mcu/src/peripheral/systick.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:36:56Z"
concept_id: crates/oxide-mcu/src/peripheral/systick/SysTick
language: rust
---

# SysTick

ARM SysTick 24-bit core system timer.

## Signature

```rust
pub struct SysTick
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

ARM SysTick 24-bit core system timer.
[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]

## Methods

- `enabled`
- `tick_interrupt`
- `clk_source_cpu`
- `count_flag`
- `reload_value`
- `current_value`

## Source
Lines 7–14 in `crates/oxide-mcu/src/peripheral/systick.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [systick](/crates/oxide-mcu/src/peripheral/systick.md) |
