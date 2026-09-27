---
okf_version: "0.2"
type: Function
title: draw_dashed_line
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/draw_dashed_line
language: rust
---

# draw_dashed_line

## Signature

```rust
fn draw_dashed_line(frame: &mut canvas::Frame, p0: Point, p1: Point, width: f32, color: Color)
```

## Source
Lines 142–169 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| called_by | [draw_line_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_line_bucket.md) |
