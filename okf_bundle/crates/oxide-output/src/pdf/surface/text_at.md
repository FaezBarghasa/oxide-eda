---
okf_version: "0.2"
type: Function
title: text_at
description: "Emit text at (x, y) with given font, size (pt), and string."
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface/text_at
language: rust
---

# text_at

Emit text at (x, y) with given font, size (pt), and string.

## Signature

```rust
impl PdfSurface { pub fn text_at(&mut self, x: f32, y: f32, font_name: &str, size_pt: f32, text: &str) }
```

## Visibility

- `pub`

## Docstring

Emit text at (x, y) with given font, size (pt), and string.
Font name should match the resource dictionary (e.g. "F1" for the first registered font).

## Source
Lines 121–128 in `crates/oxide-output/src/pdf/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-output/src/pdf/surface.md) |
| calls | [escape_pdf_string](/crates/oxide-output/src/pdf/surface/escape_pdf_string.md) |
