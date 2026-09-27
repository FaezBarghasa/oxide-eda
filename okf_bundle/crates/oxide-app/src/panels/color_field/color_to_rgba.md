---
okf_version: "0.2"
type: Function
title: color_to_rgba
description: "Quantise an `iced::Color` to 8-bit RGBA. Clamps each channel into"
resource: crates/oxide-app/src/panels/color_field.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/color_field/color_to_rgba
language: rust
---

# color_to_rgba

Quantise an `iced::Color` to 8-bit RGBA. Clamps each channel into

## Signature

```rust
fn color_to_rgba(c: Color) -> [u8; 4]
```

## Docstring

Quantise an `iced::Color` to 8-bit RGBA. Clamps each channel into
`[0, 1]` before scaling so an out-of-gamut picker value can't wrap.

## Source
Lines 266–273 in `crates/oxide-app/src/panels/color_field.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [color_field](/crates/oxide-app/src/panels/color_field.md) |
| called_by | [color_field](/crates/oxide-app/src/panels/color_field/color_field.md) |
