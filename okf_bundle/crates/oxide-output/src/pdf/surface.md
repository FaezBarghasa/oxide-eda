---
okf_version: "0.2"
type: Module
title: surface
description: "`PdfSurface` — the second render target for the schematic scene graph."
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface
language: rust
---

# surface

`PdfSurface` — the second render target for the schematic scene graph.

## Docstring

`PdfSurface` — the second render target for the schematic scene graph.

Emits PDF operators for:
- `stroke_line(x1, y1, x2, y2, width_pt)` — `m` + `l` + `S`
- `stroke_rect(x, y, w, h, width_pt)` — `re` + `S`
- `fill_rect(x, y, w, h, rgb)` — `re` + `f`
- `text_at(x, y, font_name, size_pt, text)` — `BT` + `Tf` + `Td` + `Tj` + `ET`
- `set_stroke_color(r, g, b)` — `RG`

Tracks current stroke colour/width to avoid redundant ops. Coordinates in
PDF points (bottom-left origin).

## Relationships

| Type | Target |
|------|--------|
| related | [RgbColor](/crates/oxide-output/src/pdf/surface/RgbColor.md) |
| related | [PdfSurface](/crates/oxide-output/src/pdf/surface/PdfSurface.md) |
| related | [new](/crates/oxide-output/src/pdf/surface/new.md) |
| related | [finish](/crates/oxide-output/src/pdf/surface/finish.md) |
| related | [set_stroke_color](/crates/oxide-output/src/pdf/surface/set_stroke_color.md) |
| related | [set_fill_color](/crates/oxide-output/src/pdf/surface/set_fill_color.md) |
| related | [set_stroke_width](/crates/oxide-output/src/pdf/surface/set_stroke_width.md) |
| related | [raw_operator](/crates/oxide-output/src/pdf/surface/raw_operator.md) |
| related | [stroke_line](/crates/oxide-output/src/pdf/surface/stroke_line.md) |
| related | [stroke_rect](/crates/oxide-output/src/pdf/surface/stroke_rect.md) |
| related | [fill_rect](/crates/oxide-output/src/pdf/surface/fill_rect.md) |
| related | [text_at](/crates/oxide-output/src/pdf/surface/text_at.md) |
| related | [text_at_rotated](/crates/oxide-output/src/pdf/surface/text_at_rotated.md) |
| related | [write_operator](/crates/oxide-output/src/pdf/surface/write_operator.md) |
| related | [new](/crates/oxide-output/src/pdf/surface/new.md) |
| related | [finish](/crates/oxide-output/src/pdf/surface/finish.md) |
| related | [set_stroke_color](/crates/oxide-output/src/pdf/surface/set_stroke_color.md) |
| related | [set_fill_color](/crates/oxide-output/src/pdf/surface/set_fill_color.md) |
| related | [set_stroke_width](/crates/oxide-output/src/pdf/surface/set_stroke_width.md) |
| related | [raw_operator](/crates/oxide-output/src/pdf/surface/raw_operator.md) |
| related | [stroke_line](/crates/oxide-output/src/pdf/surface/stroke_line.md) |
| related | [stroke_rect](/crates/oxide-output/src/pdf/surface/stroke_rect.md) |
| related | [fill_rect](/crates/oxide-output/src/pdf/surface/fill_rect.md) |
| related | [text_at](/crates/oxide-output/src/pdf/surface/text_at.md) |
| related | [text_at_rotated](/crates/oxide-output/src/pdf/surface/text_at_rotated.md) |
| related | [write_operator](/crates/oxide-output/src/pdf/surface/write_operator.md) |
| related | [default](/crates/oxide-output/src/pdf/surface/default.md) |
| related | [default](/crates/oxide-output/src/pdf/surface/default.md) |
| related | [escape_pdf_string](/crates/oxide-output/src/pdf/surface/escape_pdf_string.md) |
| related | [escape_pdf_string_handles_special_chars](/crates/oxide-output/src/pdf/surface/escape_pdf_string_handles_special_chars.md) |
| related | [surface_tracks_stroke_color](/crates/oxide-output/src/pdf/surface/surface_tracks_stroke_color.md) |
