---
okf_version: "0.2"
type: Function
title: get_pixel
description: "Read pixel color at (x, y)."
resource: crates/oxide-mcu/src/peripheral/display.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:49:55Z"
concept_id: crates/oxide-mcu/src/peripheral/display/get_pixel
language: rust
---

# get_pixel

Read pixel color at (x, y).

## Signature

```rust
impl DisplaySimulator { pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> }
```

## Visibility

- `pub`

## Docstring

Read pixel color at (x, y).

## Source
Lines 129–141 in `crates/oxide-mcu/src/peripheral/display.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [display](/crates/oxide-mcu/src/peripheral/display.md) |
