---
okf_version: "0.2"
type: Function
title: glyph_advance
resource: crates/oxide-output/src/svg/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/text/glyph_advance
language: rust
---

# glyph_advance

## Signature

```rust
fn glyph_advance(face: &Face<'_>, gid: GlyphId, scale: f32) -> f32
```

## Source
Lines 214–218 in `crates/oxide-output/src/svg/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-output/src/svg/text.md) |
| called_by | [draw_text_outline](/crates/oxide-output/src/svg/text/draw_text_outline.md) |
| called_by | [measure_text_advance_runs](/crates/oxide-output/src/svg/text/measure_text_advance_runs.md) |
