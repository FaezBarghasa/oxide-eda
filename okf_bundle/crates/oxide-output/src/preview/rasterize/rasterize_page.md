---
okf_version: "0.2"
type: Function
title: rasterize_page
description: Rasterise a single sheet to an RGBA bitmap.
resource: crates/oxide-output/src/preview/rasterize.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/preview/rasterize/rasterize_page
language: rust
---

# rasterize_page

Rasterise a single sheet to an RGBA bitmap.

## Signature

```rust
pub fn rasterize_page(
    sheet: &SheetSnapshot,
    page_w_mm: f64,
    page_h_mm: f64,
    opts: &PreviewOptions,
    _ctx: &ExportContext,
    expr_tables: &ExpressionTables,
) -> Option<PreviewPage>
```

## Visibility

- `pub`

## Docstring

Rasterise a single sheet to an RGBA bitmap.

Uses the same `PageTransform` (FitToPage scale + margin offset) as the PDF
exporter so the preview thumbnail matches the exported document exactly.

## Source
Lines 13–54 in `crates/oxide-output/src/preview/rasterize.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rasterize](/crates/oxide-output/src/preview/rasterize.md) |
| calls | [sheet_cell_value](/crates/oxide-output/src/expression/sheet_cell_value.md) |
| called_by | [rasterize](/crates/oxide-output/src/preview/mod/rasterize.md) |
