---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/panels/element_properties/drawing_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/drawing_preview/draw_1
language: rust
---

# draw

## Signature

```rust
fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry>
```

## Source
Lines 23–193 in `crates/oxide-app/src/panels/element_properties/drawing_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawing_preview](/crates/oxide-app/src/panels/element_properties/drawing_preview.md) |
| calls | [shape_preview_bbox](/crates/oxide-app/src/panels/element_properties/drawing_preview/shape_preview_bbox.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
| calls | [arc_sweep_local](/crates/oxide-app/src/panels/element_properties/drawing_preview/arc_sweep_local.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
