---
okf_version: "0.2"
type: Module
title: text
description: "Text rasterization — glyph outlining, markup runs, and fonts."
resource: crates/oxide-output/src/svg/text.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/text
language: rust
---

# text

Text rasterization — glyph outlining, markup runs, and fonts.

## Docstring

Text rasterization — glyph outlining, markup runs, and fonts.

Rasterizes an `SvgElement::Text` into `tiny_skia` glyph paths:
markup-run splitting (sub/superscript, overbar), advance measuring,
the embedded font faces, and the rotation-aware outline builder.

Extracted verbatim from the SVG exporter (`svg/mod.rs`); pure code
motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [TextStyle](/crates/oxide-output/src/svg/text/TextStyle.md) |
| related | [draw_text_outline](/crates/oxide-output/src/svg/text/draw_text_outline.md) |
| related | [measure_text_advance_runs](/crates/oxide-output/src/svg/text/measure_text_advance_runs.md) |
| related | [MarkupRun](/crates/oxide-output/src/svg/text/MarkupRun.md) |
| related | [markup_runs](/crates/oxide-output/src/svg/text/markup_runs.md) |
| related | [rotate_about](/crates/oxide-output/src/svg/text/rotate_about.md) |
| related | [glyph_advance](/crates/oxide-output/src/svg/text/glyph_advance.md) |
| related | [face_for_alias](/crates/oxide-output/src/svg/text/face_for_alias.md) |
| related | [TinyPathOutlineBuilder](/crates/oxide-output/src/svg/text/TinyPathOutlineBuilder.md) |
| related | [new](/crates/oxide-output/src/svg/text/new.md) |
| related | [finish](/crates/oxide-output/src/svg/text/finish.md) |
| related | [map_point](/crates/oxide-output/src/svg/text/map_point.md) |
| related | [new](/crates/oxide-output/src/svg/text/new.md) |
| related | [finish](/crates/oxide-output/src/svg/text/finish.md) |
| related | [map_point](/crates/oxide-output/src/svg/text/map_point.md) |
| related | [move_to](/crates/oxide-output/src/svg/text/move_to.md) |
| related | [line_to](/crates/oxide-output/src/svg/text/line_to.md) |
| related | [quad_to](/crates/oxide-output/src/svg/text/quad_to.md) |
| related | [curve_to](/crates/oxide-output/src/svg/text/curve_to.md) |
| related | [close](/crates/oxide-output/src/svg/text/close.md) |
| related | [move_to](/crates/oxide-output/src/svg/text/move_to.md) |
| related | [line_to](/crates/oxide-output/src/svg/text/line_to.md) |
| related | [quad_to](/crates/oxide-output/src/svg/text/quad_to.md) |
| related | [curve_to](/crates/oxide-output/src/svg/text/curve_to.md) |
| related | [close](/crates/oxide-output/src/svg/text/close.md) |
