---
okf_version: "0.2"
type: Function
title: drawing_stroke_color_row
description: Altium-style stroke colour swatch row. A small preset palette
resource: crates/oxide-app/src/panels/element_properties/drawing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/drawing/drawing_stroke_color_row
language: rust
---

# drawing_stroke_color_row

Altium-style stroke colour swatch row. A small preset palette

## Signature

```rust
fn drawing_stroke_color_row(
    current: Option<oxide_types::schematic::StrokeColor>,
    muted: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Altium-style stroke colour swatch row. A small preset palette
(Theme/Red/Green/Blue/Yellow/Orange/White/Black) lets the user
recolour a placed shape without committing to a full colour
picker. Each tile dispatches UpdateDrawingEdit::StrokeColor.

## Source
Lines 420–510 in `crates/oxide-app/src/panels/element_properties/drawing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawing](/crates/oxide-app/src/panels/element_properties/drawing.md) |
| calls | [StrokeColor](/crates/oxide-types/src/schematic/sheet/StrokeColor.md) |
| called_by | [view_drawing_properties](/crates/oxide-app/src/panels/element_properties/drawing/view_drawing_properties.md) |
