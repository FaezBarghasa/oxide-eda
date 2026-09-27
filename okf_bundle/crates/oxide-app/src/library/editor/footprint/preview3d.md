---
okf_version: "0.2"
type: Module
title: preview3d
description: Procedural 3D preview pane.
resource: crates/oxide-app/src/library/editor/footprint/preview3d.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/preview3d
language: rust
---

# preview3d

Procedural 3D preview pane.

## Docstring

Procedural 3D preview pane.

Stub-quality CPU isometric render of the footprint's pads
(extruded as boxes), the courtyard outline, and the Body3D box
straight from `Footprint::body_3d`. The render is intentionally
cheap — a single `iced::widget::Canvas` with no GPU pipeline.

TODO(v2.x): real 3D pipeline per
`docs/internal/docs/PCB_3D_RENDER_PLAN.md` (wgpu Shader widget
+ STEP geometry triangulation + lighting model).

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/footprint/preview3d/view.md) |
| related | [Preview3D](/crates/oxide-app/src/library/editor/footprint/preview3d/Preview3D.md) |
| related | [draw](/crates/oxide-app/src/library/editor/footprint/preview3d/draw.md) |
| related | [draw](/crates/oxide-app/src/library/editor/footprint/preview3d/draw.md) |
| related | [fill_quad](/crates/oxide-app/src/library/editor/footprint/preview3d/fill_quad.md) |
| related | [stroke_quad](/crates/oxide-app/src/library/editor/footprint/preview3d/stroke_quad.md) |
| related | [compute_bbox](/crates/oxide-app/src/library/editor/footprint/preview3d/compute_bbox.md) |
| related | [body_bbox](/crates/oxide-app/src/library/editor/footprint/preview3d/body_bbox.md) |
| related | [pad_color](/crates/oxide-app/src/library/editor/footprint/preview3d/pad_color.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
