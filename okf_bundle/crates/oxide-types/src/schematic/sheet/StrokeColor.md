---
okf_version: "0.2"
type: Class
title: StrokeColor
description: "Optional RGBA override for an individual `SchDrawing`. `None` means"
resource: crates/oxide-types/src/schematic/sheet.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-types/src/schematic/sheet/StrokeColor
language: rust
---

# StrokeColor

Optional RGBA override for an individual `SchDrawing`. `None` means

## Signature

```rust
pub struct StrokeColor
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Optional RGBA override for an individual `SchDrawing`. `None` means
"use the theme's default drawing colour" — the renderer falls back to
`CanvasColors.outline`. Stored per-drawing so users can recolour
individual shapes without disturbing the sheet theme.
[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]

## Methods

- `r`
- `g`
- `b`
- `a`

## Source
Lines 148–153 in `crates/oxide-types/src/schematic/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-types/src/schematic/sheet.md) |
| called_by | [drawing_stroke_color_row](/crates/oxide-app/src/panels/element_properties/drawing/drawing_stroke_color_row.md) |
