---
okf_version: "0.2"
type: Function
title: tick_second
description: Advances RTC clock by 1 second.
resource: crates/oxide-mcu/src/peripheral/rtc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:27Z"
concept_id: crates/oxide-mcu/src/peripheral/rtc/tick_second
language: rust
---

# tick_second

Advances RTC clock by 1 second.

## Signature

```rust
impl RtcPeripheral { pub fn tick_second(&mut self) }
```

## Visibility

- `pub`

## Docstring

Advances RTC clock by 1 second.

## Source
Lines 60–75 in `crates/oxide-mcu/src/peripheral/rtc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtc](/crates/oxide-mcu/src/peripheral/rtc.md) |
