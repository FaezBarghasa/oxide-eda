---
okf_version: "0.2"
type: Function
title: draw_polygons
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/draw_polygons
language: rust
---

# draw_polygons

## Signature

```rust
fn draw_polygons(
    frame: &mut canvas::Frame,
    polygons: &[GpuPolygon],
    camera: &Camera,
    bounds: Rectangle,
)
```

## Source
Lines 365–401 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
| calls | [world_to_screen](/crates/oxide-app/src/pcb_canvas/world_to_screen.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [color_from_rgba](/crates/oxide-app/src/pcb_canvas/color_from_rgba.md) |
| called_by | [draw_scene](/crates/oxide-app/src/pcb_canvas/draw_scene.md) |
