---
okf_version: "0.2"
type: Function
title: draw_polygon_bucket
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/draw_polygon_bucket
language: rust
---

# draw_polygon_bucket

## Signature

```rust
fn draw_polygon_bucket(
    frame: &mut canvas::Frame,
    polygons: &[GpuPolygon],
    world_to_screen: F,
    options: SceneDrawOptions,
)
```

## Type Parameters

- `F`

## Source
Lines 286–325 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [color_from_rgba](/crates/oxide-app/src/renderer_scene_canvas/color_from_rgba.md) |
| called_by | [draw_scene_with_world_to_screen](/crates/oxide-app/src/renderer_scene_canvas/draw_scene_with_world_to_screen.md) |
