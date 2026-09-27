---
okf_version: "0.2"
type: Function
title: get_highest_pending_irq
description: "Dispatches the highest-priority pending and enabled interrupt, if any."
resource: crates/oxide-mcu/src/peripheral/nvic.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:15:35Z"
concept_id: crates/oxide-mcu/src/peripheral/nvic/get_highest_pending_irq
language: rust
---

# get_highest_pending_irq

Dispatches the highest-priority pending and enabled interrupt, if any.

## Signature

```rust
impl Nvic { pub fn get_highest_pending_irq(&self) -> Option<usize> }
```

## Visibility

- `pub`

## Docstring

Dispatches the highest-priority pending and enabled interrupt, if any.

## Source
Lines 72–93 in `crates/oxide-mcu/src/peripheral/nvic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [nvic](/crates/oxide-mcu/src/peripheral/nvic.md) |
