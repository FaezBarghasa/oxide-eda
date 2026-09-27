---
okf_version: "0.2"
type: Function
title: world_to_screen
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/world_to_screen
language: rust
---

# world_to_screen

## Signature

```rust
fn world_to_screen(camera: &Camera, bounds: Rectangle, point: [f32; 2]) -> iced::Point
```

## Source
Lines 196–198 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
| called_by | [draw_circles](/crates/oxide-app/src/pcb_canvas/draw_circles.md) |
| called_by | [draw_lines](/crates/oxide-app/src/pcb_canvas/draw_lines.md) |
| called_by | [draw_polygons](/crates/oxide-app/src/pcb_canvas/draw_polygons.md) |
