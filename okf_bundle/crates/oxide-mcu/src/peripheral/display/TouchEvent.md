---
okf_version: "0.2"
type: Class
title: TouchEvent
description: Active Touch Event on Display surface.
resource: crates/oxide-mcu/src/peripheral/display.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:49:55Z"
concept_id: crates/oxide-mcu/src/peripheral/display/TouchEvent
language: rust
---

# TouchEvent

Active Touch Event on Display surface.

## Signature

```rust
pub struct TouchEvent
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Active Touch Event on Display surface.
[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `x`
- `y`
- `pressed`
- `finger_id`

## Source
Lines 42–47 in `crates/oxide-mcu/src/peripheral/display.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [display](/crates/oxide-mcu/src/peripheral/display.md) |
