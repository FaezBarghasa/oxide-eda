---
okf_version: "0.2"
type: Module
title: colour
description: Colour mode transformations for PDF export.
resource: crates/oxide-output/src/pdf/colour.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/colour
language: rust
---

# colour

Colour mode transformations for PDF export.

## Docstring

Colour mode transformations for PDF export.

Maps RGB colours through a transformation based on `ColourMode`:
- `Colour` — pass-through (identity)
- `Grayscale` — convert to luminance via `0.299·R + 0.587·G + 0.114·B`
- `BlackAndWhite` — strokes → black, fills → white

## Relationships

| Type | Target |
|------|--------|
| related | [ColourMap](/crates/oxide-output/src/pdf/colour/ColourMap.md) |
| related | [new](/crates/oxide-output/src/pdf/colour/new.md) |
| related | [map_rgb](/crates/oxide-output/src/pdf/colour/map_rgb.md) |
| related | [map_stroke_bw](/crates/oxide-output/src/pdf/colour/map_stroke_bw.md) |
| related | [map_fill_bw](/crates/oxide-output/src/pdf/colour/map_fill_bw.md) |
| related | [new](/crates/oxide-output/src/pdf/colour/new.md) |
| related | [map_rgb](/crates/oxide-output/src/pdf/colour/map_rgb.md) |
| related | [map_stroke_bw](/crates/oxide-output/src/pdf/colour/map_stroke_bw.md) |
| related | [map_fill_bw](/crates/oxide-output/src/pdf/colour/map_fill_bw.md) |
| related | [is_approximately_white](/crates/oxide-output/src/pdf/colour/is_approximately_white.md) |
| related | [colour_mode_colour_preserves_rgb](/crates/oxide-output/src/pdf/colour/colour_mode_colour_preserves_rgb.md) |
| related | [colour_mode_grayscale_maps_red_to_0_299](/crates/oxide-output/src/pdf/colour/colour_mode_grayscale_maps_red_to_0_299.md) |
| related | [colour_mode_grayscale_maps_green_to_0_587](/crates/oxide-output/src/pdf/colour/colour_mode_grayscale_maps_green_to_0_587.md) |
| related | [colour_mode_grayscale_maps_blue_to_0_114](/crates/oxide-output/src/pdf/colour/colour_mode_grayscale_maps_blue_to_0_114.md) |
| related | [colour_mode_bw_pushes_strokes_to_black](/crates/oxide-output/src/pdf/colour/colour_mode_bw_pushes_strokes_to_black.md) |
| related | [colour_mode_bw_fills_with_white](/crates/oxide-output/src/pdf/colour/colour_mode_bw_fills_with_white.md) |
| related | [colour_mode_bw_keeps_existing_white_white](/crates/oxide-output/src/pdf/colour/colour_mode_bw_keeps_existing_white_white.md) |
| related | [colour_mode_grayscale_white_stays_white](/crates/oxide-output/src/pdf/colour/colour_mode_grayscale_white_stays_white.md) |
| related | [colour_mode_grayscale_black_stays_black](/crates/oxide-output/src/pdf/colour/colour_mode_grayscale_black_stays_black.md) |
