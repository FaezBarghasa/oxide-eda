---
okf_version: "0.2"
type: Function
title: set_pin_output
resource: crates/oxide-mcu/src/peripheral/gpio.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:37:03Z"
concept_id: crates/oxide-mcu/src/peripheral/gpio/set_pin_output
language: rust
---

# set_pin_output

## Signature

```rust
impl GpioPort { pub fn set_pin_output(&mut self, pin_idx: usize, level: LogicLevel) }
```

## Visibility

- `pub`

## Source
Lines 72–81 in `crates/oxide-mcu/src/peripheral/gpio.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gpio](/crates/oxide-mcu/src/peripheral/gpio.md) |
