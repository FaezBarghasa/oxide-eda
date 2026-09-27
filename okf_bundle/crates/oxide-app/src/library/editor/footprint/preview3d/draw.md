---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/library/editor/footprint/preview3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/preview3d/draw
language: rust
---

# draw

## Signature

```rust
impl Preview3D<'a> { fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> }
```

## Type Parameters

- `'a`

## Source
Lines 34–240 in `crates/oxide-app/src/library/editor/footprint/preview3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview3d](/crates/oxide-app/src/library/editor/footprint/preview3d.md) |
| calls | [compute_bbox](/crates/oxide-app/src/library/editor/footprint/preview3d/compute_bbox.md) |
| calls | [project](/crates/oxide-app/src/app/state/scope/project.md) |
| calls | [fill_quad](/crates/oxide-app/src/library/editor/footprint/preview3d/fill_quad.md) |
| calls | [stroke_quad](/crates/oxide-app/src/library/editor/footprint/preview3d/stroke_quad.md) |
| calls | [pad_color](/crates/oxide-app/src/library/editor/footprint/preview3d/pad_color.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [body_bbox](/crates/oxide-app/src/library/editor/footprint/preview3d/body_bbox.md) |
