---
okf_version: "0.2"
type: Function
title: draw_lines
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/draw_lines
language: rust
---

# draw_lines

## Signature

```rust
fn draw_lines(
    frame: &mut canvas::Frame,
    lines: &[LineSegment],
    camera: &Camera,
    bounds: Rectangle,
)
```

## Source
Lines 313–337 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
| calls | [world_to_screen](/crates/oxide-app/src/pcb_canvas/world_to_screen.md) |
| calls | [color_from_rgba](/crates/oxide-app/src/pcb_canvas/color_from_rgba.md) |
| calls | [draw_dashed_line](/crates/oxide-app/src/pcb_canvas/draw_dashed_line.md) |
| called_by | [draw_scene](/crates/oxide-app/src/pcb_canvas/draw_scene.md) |
