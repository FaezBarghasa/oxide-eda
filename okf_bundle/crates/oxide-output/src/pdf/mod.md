---
okf_version: "0.2"
type: Module
title: pdf
description: "PDF export via `pdf-writer`."
resource: crates/oxide-output/src/pdf/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-output/src/pdf/mod
language: rust
---

# pdf

PDF export via `pdf-writer`.

## Docstring

PDF export via `pdf-writer`.

See `OUTPUT_PLAN.md` §3. `PdfSurface` (in `surface.rs`) acts as a second
render target for the schematic — wires, symbols, labels, title block.
Screen (Iced Canvas) and PDF share one source of truth for page layout.

## Font Strategy (v0.8)

Roboto + Iosevka TTFs are embedded at compile time (see `font.rs`) but
NOT yet emitted as Type0 composite fonts in the PDF — that's a v0.9 job.
For v0.8 every text operator references one of four aliases `/F1`–`/F4`
pointing at the PDF standard-14 Type1 fonts (Helvetica variants for
Roboto, Courier variants for Iosevka). Those standard fonts ship with
every PDF reader by spec, so exported PDFs always render text correctly
even though the glyphs come from Helvetica/Courier rather than the
bundled TTFs.

TODO(v0.9): Emit Type0 CIDFontType2 dicts with `/FontFile2` streams
pointing at the embedded TTF bytes so the exported PDFs render in the
intended Roboto/Iosevka typeface.

## Relationships

| Type | Target |
|------|--------|
| related | [PdfExporter](/crates/oxide-output/src/pdf/mod/PdfExporter.md) |
| related | [PdfOptions](/crates/oxide-output/src/pdf/mod/PdfOptions.md) |
| related | [PdfOutput](/crates/oxide-output/src/pdf/mod/PdfOutput.md) |
| related | [PageSize](/crates/oxide-output/src/pdf/mod/PageSize.md) |
| related | [Orientation](/crates/oxide-output/src/pdf/mod/Orientation.md) |
| related | [ColourMode](/crates/oxide-output/src/pdf/mod/ColourMode.md) |
| related | [PageRange](/crates/oxide-output/src/pdf/mod/PageRange.md) |
| related | [Margins](/crates/oxide-output/src/pdf/mod/Margins.md) |
| related | [PdfScale](/crates/oxide-output/src/pdf/mod/PdfScale.md) |
| related | [default](/crates/oxide-output/src/pdf/mod/default.md) |
| related | [default](/crates/oxide-output/src/pdf/mod/default.md) |
| related | [PdfError](/crates/oxide-output/src/pdf/mod/PdfError.md) |
| related | [export](/crates/oxide-output/src/pdf/mod/export.md) |
| related | [export](/crates/oxide-output/src/pdf/mod/export.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
