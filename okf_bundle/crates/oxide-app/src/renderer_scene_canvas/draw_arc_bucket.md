---
okf_version: "0.2"
type: Function
title: draw_arc_bucket
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/draw_arc_bucket
language: rust
---

# draw_arc_bucket

## Signature

```rust
fn draw_arc_bucket(
    frame: &mut canvas::Frame,
    arcs: &[Arc],
    world_to_screen: F,
    options: SceneDrawOptions,
)
```

## Type Parameters

- `F`

## Source
Lines 227–284 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [world_is_y_up](/crates/oxide-app/src/renderer_scene_canvas/world_is_y_up.md) |
| calls | [arc_screen_span](/crates/oxide-app/src/renderer_scene_canvas/arc_screen_span.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [color_from_rgba](/crates/oxide-app/src/renderer_scene_canvas/color_from_rgba.md) |
| called_by | [draw_scene_with_world_to_screen](/crates/oxide-app/src/renderer_scene_canvas/draw_scene_with_world_to_screen.md) |
