---
okf_version: "0.2"
type: Module
title: preview
description: "Print preview. See `OUTPUT_PLAN.md` §6."
resource: crates/oxide-output/src/preview/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/preview/mod
language: rust
---

# preview

Print preview. See `OUTPUT_PLAN.md` §6.

## Docstring

Print preview. See `OUTPUT_PLAN.md` §6.

Rasterises pages via `tiny-skia` into RGBA pixel buffers for on-screen
display. The same scene graph rendering logic as PDF export but to pixels
instead of PDF operators.

**Note on text rendering:** tiny-skia has no built-in text rasterisation.
We render text as hollow rectangles with estimated dimensions
(char_count × 0.6 × font_size_px) so users recognise text placement in
the preview even without glyph rendering. This is acceptable for preview
fidelity; the actual PDF renders glyphs correctly via font subsetting.

## Relationships

| Type | Target |
|------|--------|
| related | [PreviewRasterizer](/crates/oxide-output/src/preview/mod/PreviewRasterizer.md) |
| related | [PreviewOptions](/crates/oxide-output/src/preview/mod/PreviewOptions.md) |
| related | [default](/crates/oxide-output/src/preview/mod/default.md) |
| related | [default](/crates/oxide-output/src/preview/mod/default.md) |
| related | [PreviewPage](/crates/oxide-output/src/preview/mod/PreviewPage.md) |
| related | [rasterize](/crates/oxide-output/src/preview/mod/rasterize.md) |
| related | [rasterize](/crates/oxide-output/src/preview/mod/rasterize.md) |
| related | [resolve_page_range_preview](/crates/oxide-output/src/preview/mod/resolve_page_range_preview.md) |
| related | [preview_page_structure](/crates/oxide-output/src/preview/mod/preview_page_structure.md) |
| related | [preview_options_default](/crates/oxide-output/src/preview/mod/preview_options_default.md) |
