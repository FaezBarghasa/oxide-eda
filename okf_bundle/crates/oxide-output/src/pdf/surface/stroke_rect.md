---
okf_version: "0.2"
type: Function
title: stroke_rect
description: "Stroke a rectangle outline (top-left at (x, y), width w, height h)."
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface/stroke_rect
language: rust
---

# stroke_rect

Stroke a rectangle outline (top-left at (x, y), width w, height h).

## Signature

```rust
impl PdfSurface { pub fn stroke_rect(&mut self, x: f32, y: f32, w: f32, h: f32, width_pt: f32) }
```

## Visibility

- `pub`

## Docstring

Stroke a rectangle outline (top-left at (x, y), width w, height h).

## Source
Lines 102–106 in `crates/oxide-output/src/pdf/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-output/src/pdf/surface.md) |
