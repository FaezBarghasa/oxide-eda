---
okf_version: "0.2"
type: Function
title: draw_text_outline
resource: crates/oxide-output/src/svg/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/text/draw_text_outline
language: rust
---

# draw_text_outline

## Signature

```rust
pub(super) fn draw_text_outline(pixmap: &mut Pixmap, style: &TextStyle, text: &str)
```

## Visibility

- `pub(super)`

## Source
Lines 30–113 in `crates/oxide-output/src/svg/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-output/src/svg/text.md) |
| calls | [face_for_alias](/crates/oxide-output/src/svg/text/face_for_alias.md) |
| calls | [markup_runs](/crates/oxide-output/src/svg/text/markup_runs.md) |
| calls | [measure_text_advance_runs](/crates/oxide-output/src/svg/text/measure_text_advance_runs.md) |
| calls | [rgb_to_color](/crates/oxide-output/src/svg/mod/rgb_to_color.md) |
| calls | [glyph_advance](/crates/oxide-output/src/svg/text/glyph_advance.md) |
| calls | [rotate_about](/crates/oxide-output/src/svg/text/rotate_about.md) |
| called_by | [rasterize_rgba_with_colour_mode](/crates/oxide-output/src/svg/document/rasterize_rgba_with_colour_mode.md) |
