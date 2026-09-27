---
okf_version: "0.2"
type: Function
title: text_size_px
description: "Rendered em size in logical pixels, from the one shared sizing rule."
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/text_size_px
language: rust
---

# text_size_px

Rendered em size in logical pixels, from the one shared sizing rule.

## Signature

```rust
fn text_size_px(item: &TextItem, scale_px_per_mm: f32, policy: TextSizePolicy) -> f32
```

## Docstring

Rendered em size in logical pixels, from the one shared sizing rule.

This used to compute `(size_mm * scale).max(1.0)` — treating `size_mm` as
the em size and ignoring the caller's readability limits. `size_mm` is the
glyph height the sheet asks for, so that rendered every label about 28%
short of the CPU replay and broke the millimetre contract the model
guarantees (a 10 pt import stopped measuring 50 mils).

## Source
Lines 18–20 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| calls | [text_px](/crates/oxide-gfx/src/primitive/text/text_px.md) |
| called_by | [glyphon_helpers_clamp_small_text_size](/crates/oxide-gfx/src/pipeline/text/glyphon_helpers_clamp_small_text_size.md) |
| called_by | [glyphon_helpers_map_size_position_and_bounds](/crates/oxide-gfx/src/pipeline/text/glyphon_helpers_map_size_position_and_bounds.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
