---
okf_version: "0.2"
type: Function
title: rgb_to_color
resource: crates/oxide-output/src/svg/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/mod/rgb_to_color
language: rust
---

# rgb_to_color

## Signature

```rust
fn rgb_to_color(r: f32, g: f32, b: f32) -> Color
```

## Source
Lines 116–119 in `crates/oxide-output/src/svg/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [svg](/crates/oxide-output/src/svg/mod.md) |
| called_by | [rasterize_rgba_with_colour_mode](/crates/oxide-output/src/svg/document/rasterize_rgba_with_colour_mode.md) |
| called_by | [draw_text_outline](/crates/oxide-output/src/svg/text/draw_text_outline.md) |
