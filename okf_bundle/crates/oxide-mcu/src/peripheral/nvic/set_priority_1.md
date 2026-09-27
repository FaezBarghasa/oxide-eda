---
okf_version: "0.2"
type: Function
title: set_priority
resource: crates/oxide-mcu/src/peripheral/nvic.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:15:35Z"
concept_id: crates/oxide-mcu/src/peripheral/nvic/set_priority_1
language: rust
---

# set_priority

## Signature

```rust
pub fn set_priority(&mut self, irq: usize, priority: u8)
```

## Visibility

- `pub`

## Source
Lines 65–69 in `crates/oxide-mcu/src/peripheral/nvic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [nvic](/crates/oxide-mcu/src/peripheral/nvic.md) |
