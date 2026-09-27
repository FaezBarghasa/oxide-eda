---
okf_version: "0.2"
type: Class
title: ColorFieldProps
description: "Configuration for one [`color_field`]. `M` is the caller's message"
resource: crates/oxide-app/src/panels/color_field.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/color_field/ColorFieldProps
language: rust
---

# ColorFieldProps

Configuration for one [`color_field`]. `M` is the caller's message

## Signature

```rust
pub struct ColorFieldProps
```

## Type Parameters

- `'a`
- `M`

## Visibility

- `pub`

## Docstring

Configuration for one [`color_field`]. `M` is the caller's message
type; the widget is otherwise state-free — the caller owns the
open / advanced flags and feeds them back in each render.

## Methods

- `label`
- `current`
- `none_label`
- `show_palette`
- `show_advanced`
- `muted`
- `border_c`
- `on_toggle`
- `on_advanced`
- `on_cancel`
- `on_pick`
- `on_clear`

## Source
Lines 41–72 in `crates/oxide-app/src/panels/color_field.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [color_field](/crates/oxide-app/src/panels/color_field.md) |
