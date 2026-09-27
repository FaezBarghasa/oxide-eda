---
okf_version: "0.2"
type: Module
title: drawing_preview
description: "Live shape-preview canvas widget (`DrawingPreview`) shown above the"
resource: crates/oxide-app/src/panels/element_properties/drawing_preview.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/drawing_preview
language: rust
---

# drawing_preview

Live shape-preview canvas widget (`DrawingPreview`) shown above the

## Docstring

Live shape-preview canvas widget (`DrawingPreview`) shown above the
Drawing properties rows, plus its bounding-box / arc-sweep geometry
helpers (circumcircle now comes from the shared
`oxide_types::schematic::circumcircle`, #461). Moved verbatim from the
former single-file `element_properties` module.

## Relationships

| Type | Target |
|------|--------|
| related | [DrawingPreview](/crates/oxide-app/src/panels/element_properties/drawing_preview/DrawingPreview.md) |
| related | [draw](/crates/oxide-app/src/panels/element_properties/drawing_preview/draw.md) |
| related | [draw](/crates/oxide-app/src/panels/element_properties/drawing_preview/draw.md) |
| related | [shape_preview_bbox](/crates/oxide-app/src/panels/element_properties/drawing_preview/shape_preview_bbox.md) |
| related | [arc_sweep_local](/crates/oxide-app/src/panels/element_properties/drawing_preview/arc_sweep_local.md) |
