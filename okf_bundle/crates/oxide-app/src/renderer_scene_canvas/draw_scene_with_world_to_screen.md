---
okf_version: "0.2"
type: Function
title: draw_scene_with_world_to_screen
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/draw_scene_with_world_to_screen
language: rust
---

# draw_scene_with_world_to_screen

## Signature

```rust
pub fn draw_scene_with_world_to_screen(
    frame: &mut canvas::Frame,
    scene: &Scene,
    world_to_screen: F,
    options: SceneDrawOptions,
)
```

## Type Parameters

- `F`

## Visibility

- `pub`

## Source
Lines 383–427 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [draw_line_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_line_bucket.md) |
| calls | [draw_circle_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_circle_bucket.md) |
| calls | [draw_arc_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_arc_bucket.md) |
| calls | [draw_polygon_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_polygon_bucket.md) |
| calls | [draw_text_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_text_bucket.md) |
| called_by | [draw_symbol_with_renderer](/crates/oxide-app/src/library/editor/symbol/canvas/mod/draw_symbol_with_renderer.md) |
| called_by | [draw_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/mod/draw_renderer_snapshot.md) |
