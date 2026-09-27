---
okf_version: "0.2"
type: Function
title: new
description: "Build a transform for the given sheet, page dimensions, and PDF scale"
resource: crates/oxide-output/src/pdf/layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/layout/new_1
language: rust
---

# new

Build a transform for the given sheet, page dimensions, and PDF scale

## Signature

```rust
pub fn new(
        sheet: &SheetSnapshot,
        page_w_mm: f64,
        page_h_mm: f64,
        margins: &Margins,
        scale_mode: &PdfScale,
        units_per_mm: f64,
    ) -> Self
```

## Visibility

- `pub`

## Docstring

Build a transform for the given sheet, page dimensions, and PDF scale
option.

`mm_per_unit` is the number of mm in one output unit:
- PDF: `25.4 / 72.0` (one point = 1/72 inch = 25.4/72 mm)
- Pixels: `25.4 / dpi`

## Source
Lines 91–119 in `crates/oxide-output/src/pdf/layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout](/crates/oxide-output/src/pdf/layout.md) |
| calls | [schematic_bbox](/crates/oxide-output/src/pdf/layout/schematic_bbox.md) |
| calls | [fit_to_page_scale](/crates/oxide-output/src/pdf/layout/fit_to_page_scale.md) |
