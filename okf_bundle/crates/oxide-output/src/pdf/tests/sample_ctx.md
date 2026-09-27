---
okf_version: "0.2"
type: Function
title: sample_ctx
resource: crates/oxide-output/src/pdf/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/tests/sample_ctx
language: rust
---

# sample_ctx

## Signature

```rust
fn sample_ctx(sheet_count: usize) -> ExportContext
```

## Source
Lines 33–47 in `crates/oxide-output/src/pdf/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-output/src/pdf/tests.md) |
| calls | [empty_sheet](/crates/oxide-output/src/pdf/tests/empty_sheet.md) |
| called_by | [colour_mode_bw_pushes_strokes_to_black](/crates/oxide-output/src/pdf/tests/colour_mode_bw_pushes_strokes_to_black.md) |
| called_by | [colour_mode_colour_preserves_rgb](/crates/oxide-output/src/pdf/tests/colour_mode_colour_preserves_rgb.md) |
| called_by | [colour_mode_grayscale_maps_red_to_0_299](/crates/oxide-output/src/pdf/tests/colour_mode_grayscale_maps_red_to_0_299.md) |
| called_by | [exports_schematic_content](/crates/oxide-output/src/pdf/tests/exports_schematic_content.md) |
| called_by | [fit_to_page_does_not_upscale_small_content](/crates/oxide-output/src/pdf/tests/fit_to_page_does_not_upscale_small_content.md) |
| called_by | [fit_to_page_scales_large_content_down](/crates/oxide-output/src/pdf/tests/fit_to_page_scales_large_content_down.md) |
| called_by | [multi_sheet_produces_multi_page](/crates/oxide-output/src/pdf/tests/multi_sheet_produces_multi_page.md) |
| called_by | [no_outlines_emitted_when_every_bookmark_toggle_is_off](/crates/oxide-output/src/pdf/tests/no_outlines_emitted_when_every_bookmark_toggle_is_off.md) |
| called_by | [outlines_emitted_when_components_toggle_is_on](/crates/oxide-output/src/pdf/tests/outlines_emitted_when_components_toggle_is_on.md) |
| called_by | [page_paper_colour_is_filled_in_content_stream](/crates/oxide-output/src/pdf/tests/page_paper_colour_is_filled_in_content_stream.md) |
| called_by | [page_range_out_of_bounds](/crates/oxide-output/src/pdf/tests/page_range_out_of_bounds.md) |
| called_by | [page_range_range_inclusive](/crates/oxide-output/src/pdf/tests/page_range_range_inclusive.md) |
| called_by | [page_range_specific](/crates/oxide-output/src/pdf/tests/page_range_specific.md) |
| called_by | [page_size_reflected_in_media_box](/crates/oxide-output/src/pdf/tests/page_size_reflected_in_media_box.md) |
| called_by | [produces_valid_pdf_header](/crates/oxide-output/src/pdf/tests/produces_valid_pdf_header.md) |
| called_by | [template_draws_frame_rect](/crates/oxide-output/src/pdf/tests/template_draws_frame_rect.md) |
| called_by | [template_renders_substituted_text_in_title_block](/crates/oxide-output/src/pdf/tests/template_renders_substituted_text_in_title_block.md) |
