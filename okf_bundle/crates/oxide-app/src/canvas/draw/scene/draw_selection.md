---
okf_version: "0.2"
type: Function
title: draw_selection
description: Layer 3 — selection overlay + ERC markers (uncached while dragging).
resource: crates/oxide-app/src/canvas/draw/scene.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/scene/draw_selection
language: rust
---

# draw_selection

Layer 3 — selection overlay + ERC markers (uncached while dragging).

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_selection(
        &self,
        renderer: &Renderer,
        bounds: Rectangle,
        effective_snapshot: Option<&crate::schematic_runtime::SchematicRenderSnapshot>,
        drag_offset: Option<(f64, f64)>,
    ) -> Option<canvas::Geometry> }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Layer 3 — selection overlay + ERC markers (uncached while dragging).

## Source
Lines 241–306 in `crates/oxide-app/src/canvas/draw/scene.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene](/crates/oxide-app/src/canvas/draw/scene.md) |
| calls | [draw_selection_overlay](/crates/oxide-app/src/schematic_runtime/selection/draw_selection_overlay.md) |
| calls | [draw_erc_markers](/crates/oxide-app/src/schematic_runtime/overlay/draw_erc_markers.md) |
