---
okf_version: "0.2"
type: Class
title: Iwdg
description: Independent Watchdog (IWDG) clocked by dedicated low-speed oscillator (LSI ~32 kHz).
resource: crates/oxide-mcu/src/peripheral/watchdog.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:38:20Z"
concept_id: crates/oxide-mcu/src/peripheral/watchdog/Iwdg
language: rust
---

# Iwdg

Independent Watchdog (IWDG) clocked by dedicated low-speed oscillator (LSI ~32 kHz).

## Signature

```rust
pub struct Iwdg
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Independent Watchdog (IWDG) clocked by dedicated low-speed oscillator (LSI ~32 kHz).
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `enabled`
- `prescaler`
- `reload_value`
- `counter`
- `reset_tripped`

## Source
Lines 7–13 in `crates/oxide-mcu/src/peripheral/watchdog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [watchdog](/crates/oxide-mcu/src/peripheral/watchdog.md) |
