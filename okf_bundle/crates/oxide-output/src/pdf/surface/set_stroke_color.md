---
okf_version: "0.2"
type: Function
title: set_stroke_color
description: "Set stroke color (0.0-1.0 per channel). Emits `RG` operator only if changed."
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface/set_stroke_color
language: rust
---

# set_stroke_color

Set stroke color (0.0-1.0 per channel). Emits `RG` operator only if changed.

## Signature

```rust
impl PdfSurface { pub fn set_stroke_color(&mut self, r: f32, g: f32, b: f32) }
```

## Visibility

- `pub`

## Docstring

Set stroke color (0.0-1.0 per channel). Emits `RG` operator only if changed.

## Source
Lines 55–65 in `crates/oxide-output/src/pdf/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-output/src/pdf/surface.md) |
