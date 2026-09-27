---
okf_version: "0.2"
type: Function
title: draw_line_bucket
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/draw_line_bucket
language: rust
---

# draw_line_bucket

## Signature

```rust
fn draw_line_bucket(
    frame: &mut canvas::Frame,
    lines: &[LineSegment],
    world_to_screen: F,
    options: SceneDrawOptions,
)
```

## Type Parameters

- `F`

## Source
Lines 171–198 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [color_from_rgba](/crates/oxide-app/src/renderer_scene_canvas/color_from_rgba.md) |
| calls | [draw_dashed_line](/crates/oxide-app/src/renderer_scene_canvas/draw_dashed_line.md) |
| called_by | [draw_scene_with_world_to_screen](/crates/oxide-app/src/renderer_scene_canvas/draw_scene_with_world_to_screen.md) |
