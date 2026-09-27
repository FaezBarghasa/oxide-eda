---
okf_version: "0.2"
type: Function
title: step_ticks
description: Advance watchdog by LSI clock ticks.
resource: crates/oxide-mcu/src/peripheral/watchdog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:20Z"
concept_id: crates/oxide-mcu/src/peripheral/watchdog/step_ticks
language: rust
---

# step_ticks

Advance watchdog by LSI clock ticks.

## Signature

```rust
impl Iwdg { pub fn step_ticks(&mut self, ticks: u16) -> bool }
```

## Visibility

- `pub`

## Docstring

Advance watchdog by LSI clock ticks.

## Source
Lines 47–60 in `crates/oxide-mcu/src/peripheral/watchdog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [watchdog](/crates/oxide-mcu/src/peripheral/watchdog.md) |
