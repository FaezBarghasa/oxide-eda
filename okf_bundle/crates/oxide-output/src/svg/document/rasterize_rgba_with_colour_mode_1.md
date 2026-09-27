---
okf_version: "0.2"
type: Function
title: rasterize_rgba_with_colour_mode
resource: crates/oxide-output/src/svg/document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/document/rasterize_rgba_with_colour_mode_1
language: rust
---

# rasterize_rgba_with_colour_mode

## Signature

```rust
pub fn rasterize_rgba_with_colour_mode(
        &self,
        width: u32,
        height: u32,
        colour_mode: ColourMode,
    ) -> Option<Vec<u8>>
```

## Visibility

- `pub`

## Source
Lines 392–458 in `crates/oxide-output/src/svg/document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document](/crates/oxide-output/src/svg/document.md) |
| calls | [path_to_tiny_skia](/crates/oxide-output/src/svg/geometry/path_to_tiny_skia.md) |
| calls | [map_colour_mode](/crates/oxide-output/src/svg/mod/map_colour_mode.md) |
| calls | [rgb_to_color](/crates/oxide-output/src/svg/mod/rgb_to_color.md) |
| calls | [draw_text_outline](/crates/oxide-output/src/svg/text/draw_text_outline.md) |
