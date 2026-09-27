---
okf_version: "0.2"
type: Module
title: svg
description: "Schematic -> SVG path-based intermediate context."
resource: crates/oxide-output/src/svg/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/mod
language: rust
---

# svg

Schematic -> SVG path-based intermediate context.

## Docstring

Schematic -> SVG path-based intermediate context.

This module is the canonical geometry bridge:
schematic page -> SVG path elements -> PDF / preview backends.

## Relationships

| Type | Target |
|------|--------|
| related | [SvgTextAlign](/crates/oxide-output/src/svg/mod/SvgTextAlign.md) |
| related | [SvgTextVAlign](/crates/oxide-output/src/svg/mod/SvgTextVAlign.md) |
| related | [SvgPoint](/crates/oxide-output/src/svg/mod/SvgPoint.md) |
| related | [SvgPathCommand](/crates/oxide-output/src/svg/mod/SvgPathCommand.md) |
| related | [SvgStyle](/crates/oxide-output/src/svg/mod/SvgStyle.md) |
| related | [SvgElement](/crates/oxide-output/src/svg/mod/SvgElement.md) |
| related | [SvgRenderContext](/crates/oxide-output/src/svg/mod/SvgRenderContext.md) |
| related | [SvgEvaluatorInputs](/crates/oxide-output/src/svg/mod/SvgEvaluatorInputs.md) |
| related | [fill_to_rgb](/crates/oxide-output/src/svg/mod/fill_to_rgb.md) |
| related | [normalize_standard_text](/crates/oxide-output/src/svg/mod/normalize_standard_text.md) |
| related | [normalize_standard_text_with_ctx](/crates/oxide-output/src/svg/mod/normalize_standard_text_with_ctx.md) |
| related | [rgb_to_color](/crates/oxide-output/src/svg/mod/rgb_to_color.md) |
| related | [map_colour_mode](/crates/oxide-output/src/svg/mod/map_colour_mode.md) |
| related | [pt](/crates/oxide-output/src/svg/mod/pt.md) |
| related | [empty_sheet_snapshot](/crates/oxide-output/src/svg/mod/empty_sheet_snapshot.md) |
| related | [svg_document_is_emitted](/crates/oxide-output/src/svg/mod/svg_document_is_emitted.md) |
| related | [can_rasterize_context](/crates/oxide-output/src/svg/mod/can_rasterize_context.md) |
