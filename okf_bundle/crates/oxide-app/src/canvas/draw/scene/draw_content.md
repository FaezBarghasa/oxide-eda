---
okf_version: "0.2"
type: Function
title: draw_content
description: Layer 2 — the schematic content (cached unless panning/dragging).
resource: crates/oxide-app/src/canvas/draw/scene.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/scene/draw_content
language: rust
---

# draw_content

Layer 2 — the schematic content (cached unless panning/dragging).

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_content(
        &self,
        state: &CanvasState,
        renderer: &Renderer,
        bounds: Rectangle,
        effective_snapshot: Option<&crate::schematic_runtime::SchematicRenderSnapshot>,
        drag_offset: Option<(f64, f64)>,
    ) -> canvas::Geometry }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Layer 2 — the schematic content (cached unless panning/dragging).

## Source
Lines 5–64 in `crates/oxide-app/src/canvas/draw/scene.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene](/crates/oxide-app/src/canvas/draw/scene.md) |
| calls | [render_schematic](/crates/oxide-app/src/schematic_runtime/mod/render_schematic.md) |
