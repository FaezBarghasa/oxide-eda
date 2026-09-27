---
okf_version: "0.2"
type: Module
title: text
description: Text pipeline foundation.
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text
language: rust
---

# text

Text pipeline foundation.

## Docstring

Text pipeline foundation.

CLEAN ROOM DECLARATION
This module was written without reference to GPL-licensed software.
Sources: IPC-2612-1, IEEE 315, IEC 60617, wgpu/WGSL public docs.

## Relationships

| Type | Target |
|------|--------|
| related | [text_size_px](/crates/oxide-gfx/src/pipeline/text/text_size_px.md) |
| related | [text_position_px](/crates/oxide-gfx/src/pipeline/text/text_position_px.md) |
| related | [normalize_rotation_radians](/crates/oxide-gfx/src/pipeline/text/normalize_rotation_radians.md) |
| related | [alignment_offset_px](/crates/oxide-gfx/src/pipeline/text/alignment_offset_px.md) |
| related | [rotated_offset_px](/crates/oxide-gfx/src/pipeline/text/rotated_offset_px.md) |
| related | [anchored_top_left_px](/crates/oxide-gfx/src/pipeline/text/anchored_top_left_px.md) |
| related | [RectPx](/crates/oxide-gfx/src/pipeline/text/RectPx.md) |
| related | [rect_from_top_left_size](/crates/oxide-gfx/src/pipeline/text/rect_from_top_left_size.md) |
| related | [rect_intersects_viewport](/crates/oxide-gfx/src/pipeline/text/rect_intersects_viewport.md) |
| related | [rect_overlap_area_px](/crates/oxide-gfx/src/pipeline/text/rect_overlap_area_px.md) |
| related | [rect_area_px](/crates/oxide-gfx/src/pipeline/text/rect_area_px.md) |
| related | [overlap_ratio_by_smaller_area](/crates/oxide-gfx/src/pipeline/text/overlap_ratio_by_smaller_area.md) |
| related | [viewport_bounds](/crates/oxide-gfx/src/pipeline/text/viewport_bounds.md) |
| related | [measure_text_bounds_px](/crates/oxide-gfx/src/pipeline/text/measure_text_bounds_px.md) |
| related | [attrs_for_item](/crates/oxide-gfx/src/pipeline/text/attrs_for_item.md) |
| related | [to_glyphon_color](/crates/oxide-gfx/src/pipeline/text/to_glyphon_color.md) |
| related | [GlyphonPreparedText](/crates/oxide-gfx/src/pipeline/text/GlyphonPreparedText.md) |
| related | [GlyphonTextPipeline](/crates/oxide-gfx/src/pipeline/text/GlyphonTextPipeline.md) |
| related | [new](/crates/oxide-gfx/src/pipeline/text/new.md) |
| related | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
| related | [draw](/crates/oxide-gfx/src/pipeline/text/draw.md) |
| related | [trim_atlas](/crates/oxide-gfx/src/pipeline/text/trim_atlas.md) |
| related | [text_count](/crates/oxide-gfx/src/pipeline/text/text_count.md) |
| related | [viewport_size_px](/crates/oxide-gfx/src/pipeline/text/viewport_size_px.md) |
| related | [new](/crates/oxide-gfx/src/pipeline/text/new.md) |
| related | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
| related | [draw](/crates/oxide-gfx/src/pipeline/text/draw.md) |
| related | [trim_atlas](/crates/oxide-gfx/src/pipeline/text/trim_atlas.md) |
| related | [text_count](/crates/oxide-gfx/src/pipeline/text/text_count.md) |
| related | [viewport_size_px](/crates/oxide-gfx/src/pipeline/text/viewport_size_px.md) |
| related | [glyphon_helpers_map_size_position_and_bounds](/crates/oxide-gfx/src/pipeline/text/glyphon_helpers_map_size_position_and_bounds.md) |
| related | [glyphon_helpers_clamp_small_text_size](/crates/oxide-gfx/src/pipeline/text/glyphon_helpers_clamp_small_text_size.md) |
| related | [glyphon_helpers_map_style_and_color](/crates/oxide-gfx/src/pipeline/text/glyphon_helpers_map_style_and_color.md) |
| related | [alignment_helpers_map_offsets](/crates/oxide-gfx/src/pipeline/text/alignment_helpers_map_offsets.md) |
| related | [anchored_position_respects_alignment_and_rotation](/crates/oxide-gfx/src/pipeline/text/anchored_position_respects_alignment_and_rotation.md) |
| related | [rotation_normalization_is_stable](/crates/oxide-gfx/src/pipeline/text/rotation_normalization_is_stable.md) |
| related | [clipping_helper_rejects_outside_rectangles](/crates/oxide-gfx/src/pipeline/text/clipping_helper_rejects_outside_rectangles.md) |
| related | [overlap_ratio_detects_dense_overlap](/crates/oxide-gfx/src/pipeline/text/overlap_ratio_detects_dense_overlap.md) |
