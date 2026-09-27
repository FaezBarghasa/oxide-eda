---
okf_version: "0.2"
type: Module
title: font
description: Font embedding + subsetting for PDFs.
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font
language: rust
---

# font

Font embedding + subsetting for PDFs.

## Docstring

Font embedding + subsetting for PDFs.

v0.8 references PDF standard-14 Type1 fonts (Helvetica variants + Courier
variants) via short aliases `F1`–`F4`. The alias-to-`/BaseFont` mapping is
emitted once per PDF; every page points at the same four font objects.

The Roboto + Iosevka TTF bytes below are bundled at compile time but NOT
yet wired into the PDF pipeline — they're parked for v0.9 when the Type0
composite-font dict + FontFile2 stream emission lands. The module-wide
`#![expect(dead_code)]` below is deliberate, and it expires on its own:
once the pipeline uses these, `unfulfilled_lint_expectations` says so.

## Relationships

| Type | Target |
|------|--------|
| related | [PdfFont](/crates/oxide-output/src/pdf/font/PdfFont.md) |
| related | [for_style](/crates/oxide-output/src/pdf/font/for_style.md) |
| related | [alias](/crates/oxide-output/src/pdf/font/alias.md) |
| related | [standard_ps_name](/crates/oxide-output/src/pdf/font/standard_ps_name.md) |
| related | [base_name](/crates/oxide-output/src/pdf/font/base_name.md) |
| related | [font_bytes](/crates/oxide-output/src/pdf/font/font_bytes.md) |
| related | [face](/crates/oxide-output/src/pdf/font/face.md) |
| related | [for_style](/crates/oxide-output/src/pdf/font/for_style.md) |
| related | [alias](/crates/oxide-output/src/pdf/font/alias.md) |
| related | [standard_ps_name](/crates/oxide-output/src/pdf/font/standard_ps_name.md) |
| related | [base_name](/crates/oxide-output/src/pdf/font/base_name.md) |
| related | [font_bytes](/crates/oxide-output/src/pdf/font/font_bytes.md) |
| related | [face](/crates/oxide-output/src/pdf/font/face.md) |
| related | [font_for_alias](/crates/oxide-output/src/pdf/font/font_for_alias.md) |
| related | [best_alias_for_text](/crates/oxide-output/src/pdf/font/best_alias_for_text.md) |
| related | [sanitize_pdf_text](/crates/oxide-output/src/pdf/font/sanitize_pdf_text.md) |
| related | [text_advance_pt](/crates/oxide-output/src/pdf/font/text_advance_pt.md) |
| related | [glyph_coverage](/crates/oxide-output/src/pdf/font/glyph_coverage.md) |
| related | [FontCatalog](/crates/oxide-output/src/pdf/font/FontCatalog.md) |
| related | [new](/crates/oxide-output/src/pdf/font/new.md) |
| related | [register](/crates/oxide-output/src/pdf/font/register.md) |
| related | [get](/crates/oxide-output/src/pdf/font/get.md) |
| related | [iter](/crates/oxide-output/src/pdf/font/iter.md) |
| related | [font_data](/crates/oxide-output/src/pdf/font/font_data.md) |
| related | [new](/crates/oxide-output/src/pdf/font/new.md) |
| related | [register](/crates/oxide-output/src/pdf/font/register.md) |
| related | [get](/crates/oxide-output/src/pdf/font/get.md) |
| related | [iter](/crates/oxide-output/src/pdf/font/iter.md) |
| related | [font_data](/crates/oxide-output/src/pdf/font/font_data.md) |
| related | [font_style_maps_to_embedded_variants](/crates/oxide-output/src/pdf/font/font_style_maps_to_embedded_variants.md) |
| related | [base_names_correct](/crates/oxide-output/src/pdf/font/base_names_correct.md) |
| related | [aliases_and_standard_fallbacks](/crates/oxide-output/src/pdf/font/aliases_and_standard_fallbacks.md) |
| related | [font_bytes_embedded](/crates/oxide-output/src/pdf/font/font_bytes_embedded.md) |
| related | [font_catalog_registers_unique](/crates/oxide-output/src/pdf/font/font_catalog_registers_unique.md) |
| related | [font_catalog_get_retrieves](/crates/oxide-output/src/pdf/font/font_catalog_get_retrieves.md) |
| related | [fonts_parse_successfully](/crates/oxide-output/src/pdf/font/fonts_parse_successfully.md) |
| related | [embeds_roboto_font](/crates/oxide-output/src/pdf/font/embeds_roboto_font.md) |
| related | [font_bytes_are_valid_ttf](/crates/oxide-output/src/pdf/font/font_bytes_are_valid_ttf.md) |
| related | [all_fonts_have_metrics](/crates/oxide-output/src/pdf/font/all_fonts_have_metrics.md) |
