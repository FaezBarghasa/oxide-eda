---
okf_version: "0.2"
type: Function
title: fit_to_page_scale
description: Scale factor that fits the content bounding box into the printable area
resource: crates/oxide-output/src/pdf/layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/layout/fit_to_page_scale
language: rust
---

# fit_to_page_scale

Scale factor that fits the content bounding box into the printable area

## Signature

```rust
pub fn fit_to_page_scale(
    page_w_mm: f64,
    page_h_mm: f64,
    margins: &Margins,
    bbox: (f64, f64, f64, f64),
) -> f64
```

## Visibility

- `pub`

## Docstring

Scale factor that fits the content bounding box into the printable area
(page minus margins).

- Never upscales beyond `1.0`.
- Treats content width/height < 1 mm as 1 mm to avoid division by zero.

## Source
Lines 53–67 in `crates/oxide-output/src/pdf/layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout](/crates/oxide-output/src/pdf/layout.md) |
| called_by | [new](/crates/oxide-output/src/pdf/layout/new.md) |
