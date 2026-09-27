---
okf_version: "0.2"
type: Function
title: draw_scene
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/draw_scene
language: rust
---

# draw_scene

## Signature

```rust
fn draw_scene(frame: &mut canvas::Frame, scene: &Scene, camera: &Camera, bounds: Rectangle)
```

## Source
Lines 403–428 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
| calls | [draw_lines](/crates/oxide-app/src/pcb_canvas/draw_lines.md) |
| calls | [draw_circles](/crates/oxide-app/src/pcb_canvas/draw_circles.md) |
| calls | [draw_polygons](/crates/oxide-app/src/pcb_canvas/draw_polygons.md) |
| called_by | [draw](/crates/oxide-app/src/pcb_canvas/draw.md) |
