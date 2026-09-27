---
okf_version: "0.2"
type: Function
title: inject_touch
description: Injects touch screen interaction event.
resource: crates/oxide-mcu/src/peripheral/display.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:49:55Z"
concept_id: crates/oxide-mcu/src/peripheral/display/inject_touch
language: rust
---

# inject_touch

Injects touch screen interaction event.

## Signature

```rust
impl DisplaySimulator { pub fn inject_touch(&mut self, x: u16, y: u16, pressed: bool) }
```

## Visibility

- `pub`

## Docstring

Injects touch screen interaction event.

## Source
Lines 144–153 in `crates/oxide-mcu/src/peripheral/display.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [display](/crates/oxide-mcu/src/peripheral/display.md) |
