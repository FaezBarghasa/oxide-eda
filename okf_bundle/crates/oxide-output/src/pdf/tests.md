---
okf_version: "0.2"
type: Module
title: tests
description: Tests for the PDF exporter.
resource: crates/oxide-output/src/pdf/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/tests
language: rust
---

# tests

Tests for the PDF exporter.

## Docstring

Tests for the PDF exporter.

## Relationships

| Type | Target |
|------|--------|
| related | [empty_sheet](/crates/oxide-output/src/pdf/tests/empty_sheet.md) |
| related | [sample_ctx](/crates/oxide-output/src/pdf/tests/sample_ctx.md) |
| related | [produces_valid_pdf_header](/crates/oxide-output/src/pdf/tests/produces_valid_pdf_header.md) |
| related | [multi_sheet_produces_multi_page](/crates/oxide-output/src/pdf/tests/multi_sheet_produces_multi_page.md) |
| related | [empty_context_errors](/crates/oxide-output/src/pdf/tests/empty_context_errors.md) |
| related | [page_range_specific](/crates/oxide-output/src/pdf/tests/page_range_specific.md) |
| related | [page_range_range_inclusive](/crates/oxide-output/src/pdf/tests/page_range_range_inclusive.md) |
| related | [page_range_out_of_bounds](/crates/oxide-output/src/pdf/tests/page_range_out_of_bounds.md) |
| related | [page_size_reflected_in_media_box](/crates/oxide-output/src/pdf/tests/page_size_reflected_in_media_box.md) |
| related | [exports_schematic_content](/crates/oxide-output/src/pdf/tests/exports_schematic_content.md) |
| related | [colour_mode_colour_preserves_rgb](/crates/oxide-output/src/pdf/tests/colour_mode_colour_preserves_rgb.md) |
| related | [colour_mode_grayscale_maps_red_to_0_299](/crates/oxide-output/src/pdf/tests/colour_mode_grayscale_maps_red_to_0_299.md) |
| related | [colour_mode_bw_pushes_strokes_to_black](/crates/oxide-output/src/pdf/tests/colour_mode_bw_pushes_strokes_to_black.md) |
| related | [fit_to_page_scales_large_content_down](/crates/oxide-output/src/pdf/tests/fit_to_page_scales_large_content_down.md) |
| related | [fit_to_page_does_not_upscale_small_content](/crates/oxide-output/src/pdf/tests/fit_to_page_does_not_upscale_small_content.md) |
| related | [template_draws_frame_rect](/crates/oxide-output/src/pdf/tests/template_draws_frame_rect.md) |
| related | [template_renders_substituted_text_in_title_block](/crates/oxide-output/src/pdf/tests/template_renders_substituted_text_in_title_block.md) |
| related | [no_outlines_emitted_when_every_bookmark_toggle_is_off](/crates/oxide-output/src/pdf/tests/no_outlines_emitted_when_every_bookmark_toggle_is_off.md) |
| related | [page_paper_colour_is_filled_in_content_stream](/crates/oxide-output/src/pdf/tests/page_paper_colour_is_filled_in_content_stream.md) |
| related | [outlines_emitted_when_components_toggle_is_on](/crates/oxide-output/src/pdf/tests/outlines_emitted_when_components_toggle_is_on.md) |
