---
okf_version: "0.2"
type: Function
title: append_stroke
description: "Stroke outline: one width-expanded quad per edge of the closed contour"
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/append_stroke
language: rust
---

# append_stroke

Stroke outline: one width-expanded quad per edge of the closed contour

## Signature

```rust
fn append_stroke(vertices: &mut Vec<PolygonVertex>, polygon: &GpuPolygon)
```

## Docstring

Stroke outline: one width-expanded quad per edge of the closed contour
(including the closing edge, `last -> first`), so fill-only-invisible areas
such as rule/keepout zones still read as an outline. World-space width in
mm — the shader's ortho projection scales it with zoom, mirroring the CPU's
`stroke_width * camera.scale`. No miter joins (thin strokes only) and no
screen-space minimum width; both are shared with the CPU path within
tolerance for the sub-0.1 mm widths the renderer emits. No-op when the
polygon has no stroke colour or a non-positive width.

## Source
Lines 112–130 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| calls | [append_edge_quad](/crates/oxide-gfx/src/pipeline/polygon/append_edge_quad.md) |
| called_by | [triangulate_polygons](/crates/oxide-gfx/src/pipeline/polygon/triangulate_polygons.md) |
