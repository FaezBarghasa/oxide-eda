---
okf_version: "0.2"
type: Function
title: set_pixel
description: "Set pixel color at (x, y)."
resource: crates/oxide-mcu/src/peripheral/display.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:49:55Z"
concept_id: crates/oxide-mcu/src/peripheral/display/set_pixel
language: rust
---

# set_pixel

Set pixel color at (x, y).

## Signature

```rust
impl DisplaySimulator { pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) }
```

## Visibility

- `pub`

## Docstring

Set pixel color at (x, y).

## Source
Lines 118–126 in `crates/oxide-mcu/src/peripheral/display.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [display](/crates/oxide-mcu/src/peripheral/display.md) |
