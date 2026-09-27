---
okf_version: "0.2"
type: Function
title: stroke_line
description: "Stroke a line from (x1, y1) to (x2, y2)."
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface/stroke_line
language: rust
---

# stroke_line

Stroke a line from (x1, y1) to (x2, y2).

## Signature

```rust
impl PdfSurface { pub fn stroke_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width_pt: f32) }
```

## Visibility

- `pub`

## Docstring

Stroke a line from (x1, y1) to (x2, y2).

## Source
Lines 94–99 in `crates/oxide-output/src/pdf/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-output/src/pdf/surface.md) |
