---
okf_version: "0.2"
type: Class
title: RtcPeripheral
description: Real-Time Clock Hardware peripheral model.
resource: crates/oxide-mcu/src/peripheral/rtc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:27Z"
concept_id: crates/oxide-mcu/src/peripheral/rtc/RtcPeripheral
language: rust
---

# RtcPeripheral

Real-Time Clock Hardware peripheral model.

## Signature

```rust
pub struct RtcPeripheral
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Real-Time Clock Hardware peripheral model.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `time`
- `alarm_a`
- `alarm_b`
- `alarm_tripped`

## Source
Lines 26–32 in `crates/oxide-mcu/src/peripheral/rtc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtc](/crates/oxide-mcu/src/peripheral/rtc.md) |
