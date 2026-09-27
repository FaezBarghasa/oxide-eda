---
okf_version: "0.2"
type: Class
title: GlyphonTextPipeline
description: "Production text path using glyphon atlas, shaping, and cached glyph rendering."
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/GlyphonTextPipeline
language: rust
---

# GlyphonTextPipeline

Production text path using glyphon atlas, shaping, and cached glyph rendering.

## Signature

```rust
pub struct GlyphonTextPipeline
```

## Visibility

- `pub`

## Docstring

Production text path using glyphon atlas, shaping, and cached glyph rendering.
The `FontSystem` is **not** owned here — [`Self::upload`] borrows the
caller's. Owning one meant this pipeline shaped text against a font system
that had never been given the app's faces, so it could only ever fall back
to something else, and its fallback chain for non-Latin content could
diverge from the CPU path's silently.

## Methods

- `swash_cache`
- `viewport`
- `atlas`
- `text_renderer`
- `buffers`
- `prepared_texts`
- `text_count`
- `viewport_size_px`

## Source
Lines 214–223 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
