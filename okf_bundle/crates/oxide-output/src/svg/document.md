---
okf_version: "0.2"
type: Module
title: document
description: "The `SvgRenderContext` public surface and document serializers."
resource: crates/oxide-output/src/svg/document.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/document
language: rust
---

# document

The `SvgRenderContext` public surface and document serializers.

## Docstring

The `SvgRenderContext` public surface and document serializers.

`from_sheet` composes a schematic page into the `SvgElement` list
(page fill, wires/buses/junctions, labels, notes, child sheets,
drawings, symbols); `rasterize_rgba*` renders that list to RGBA
pixels; `encode_svg_document` serializes it to an SVG string.

Extracted verbatim from the SVG exporter (`svg/mod.rs`); pure code
motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
| related | [rasterize_rgba](/crates/oxide-output/src/svg/document/rasterize_rgba.md) |
| related | [rasterize_rgba_with_colour_mode](/crates/oxide-output/src/svg/document/rasterize_rgba_with_colour_mode.md) |
| related | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
| related | [rasterize_rgba](/crates/oxide-output/src/svg/document/rasterize_rgba.md) |
| related | [rasterize_rgba_with_colour_mode](/crates/oxide-output/src/svg/document/rasterize_rgba_with_colour_mode.md) |
| related | [encode_svg_document](/crates/oxide-output/src/svg/document/encode_svg_document.md) |
| related | [to_svg_path_d](/crates/oxide-output/src/svg/document/to_svg_path_d.md) |
| related | [rgb_css](/crates/oxide-output/src/svg/document/rgb_css.md) |
| related | [escape_xml](/crates/oxide-output/src/svg/document/escape_xml.md) |
