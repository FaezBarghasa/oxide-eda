---
okf_version: "0.2"
type: Function
title: clear
description: Clears the framebuffer.
resource: crates/oxide-mcu/src/peripheral/display.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:49:55Z"
concept_id: crates/oxide-mcu/src/peripheral/display/clear
language: rust
---

# clear

Clears the framebuffer.

## Signature

```rust
impl DisplaySimulator { pub fn clear(&mut self, r: u8, g: u8, b: u8) }
```

## Visibility

- `pub`

## Docstring

Clears the framebuffer.

## Source
Lines 161–167 in `crates/oxide-mcu/src/peripheral/display.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [display](/crates/oxide-mcu/src/peripheral/display.md) |
