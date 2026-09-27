---
okf_version: "0.2"
type: Function
title: color_from_rgba
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/color_from_rgba
language: rust
---

# color_from_rgba

## Signature

```rust
fn color_from_rgba(rgba: [f32; 4]) -> Color
```

## Source
Lines 138–140 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| called_by | [draw_arc_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_arc_bucket.md) |
| called_by | [draw_circle_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_circle_bucket.md) |
| called_by | [draw_line_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_line_bucket.md) |
| called_by | [draw_polygon_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_polygon_bucket.md) |
| called_by | [draw_text_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_text_bucket.md) |
