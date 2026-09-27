---
okf_version: "0.2"
type: Class
title: DisplaySimulator
description: Universal Virtual Display Simulation Buffer.
resource: crates/oxide-mcu/src/peripheral/display.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-mcu"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:49:55Z"
concept_id: crates/oxide-mcu/src/peripheral/display/DisplaySimulator
language: rust
---

# DisplaySimulator

Universal Virtual Display Simulation Buffer.

## Signature

```rust
pub struct DisplaySimulator
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Universal Virtual Display Simulation Buffer.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `display_type`
- `touch_type`
- `width`
- `height`
- `framebuffer_rgba`
- `backlight_brightness`
- `backlight_on`
- `active_touch`
- `text_buffer`

## Source
Lines 51–65 in `crates/oxide-mcu/src/peripheral/display.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [display](/crates/oxide-mcu/src/peripheral/display.md) |
