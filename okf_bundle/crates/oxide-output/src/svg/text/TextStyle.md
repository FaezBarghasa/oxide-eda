---
okf_version: "0.2"
type: Class
title: TextStyle
description: "How a text run is placed and painted — everything `draw_text_outline`"
resource: crates/oxide-output/src/svg/text.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/text/TextStyle
language: rust
---

# TextStyle

How a text run is placed and painted — everything `draw_text_outline`

## Signature

```rust
pub(super) struct TextStyle
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

How a text run is placed and painted — everything `draw_text_outline`
needs besides the render target (`pixmap`) and the string content
(`text`). Built once per `SvgElement::Text` in `document.rs` and
passed by reference since the drawing routine only reads it.

## Methods

- `x`
- `y`
- `size_pt`
- `align`
- `v_align`
- `rotation_deg`
- `fill_rgb`
- `font_alias`

## Source
Lines 19–28 in `crates/oxide-output/src/svg/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-output/src/svg/text.md) |
