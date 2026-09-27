---
okf_version: "0.2"
type: Module
title: palette
description: Schematic colour palette for the PDF / preview pipeline.
resource: crates/oxide-output/src/pdf/palette.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/palette
language: rust
---

# palette

Schematic colour palette for the PDF / preview pipeline.

## Docstring

Schematic colour palette for the PDF / preview pipeline.

The on-screen schematic is themed via `oxide_types::CanvasColors`.
For the exported PDF and the preview rasteriser we lift those
values into f32 RGB triples — that's what `SvgRenderContext`
consumes for stroke / fill state. Mapping happens once when the
Print Preview modal opens (or when an export is triggered) so
every wire / symbol / label in the resulting PDF matches what the
user is looking at on the canvas.

Pre-existing tests + the empty `PdfOptions::default()` keep using
the legacy `SchematicPalette::classic()` (cream paper / dark-blue
wires / mustard symbols) so historical /Page byte snapshots stay
stable. The unified Print Preview hands the active theme's
palette in when it kicks off an export, so users see Altium-style
cream / Catppuccin Mocha / etc. honoured on paper too.

## Relationships

| Type | Target |
|------|--------|
| related | [SchematicPalette](/crates/oxide-output/src/pdf/palette/SchematicPalette.md) |
| related | [classic](/crates/oxide-output/src/pdf/palette/classic.md) |
| related | [classic](/crates/oxide-output/src/pdf/palette/classic.md) |
| related | [default](/crates/oxide-output/src/pdf/palette/default.md) |
| related | [default](/crates/oxide-output/src/pdf/palette/default.md) |
| related | [from](/crates/oxide-output/src/pdf/palette/from.md) |
| related | [from](/crates/oxide-output/src/pdf/palette/from.md) |
| related | [from](/crates/oxide-output/src/pdf/palette/from.md) |
| related | [from](/crates/oxide-output/src/pdf/palette/from.md) |
| related | [rgb](/crates/oxide-output/src/pdf/palette/rgb.md) |
| related | [classic_palette_matches_legacy_constants](/crates/oxide-output/src/pdf/palette/classic_palette_matches_legacy_constants.md) |
| related | [from_canvas_colors_normalises_u8_rgb](/crates/oxide-output/src/pdf/palette/from_canvas_colors_normalises_u8_rgb.md) |
| related | [altium_dark_paper_is_dark](/crates/oxide-output/src/pdf/palette/altium_dark_paper_is_dark.md) |
