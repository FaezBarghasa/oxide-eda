---
okf_version: "0.2"
type: Function
title: arc_screen_span
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/arc_screen_span
language: rust
---

# arc_screen_span

## Signature

```rust
pub fn arc_screen_span(start_angle: f32, end_angle: f32, world_is_y_up: bool) -> ArcScreenSpan
```

## Visibility

- `pub`

## Source
Lines 96–118 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [arc_is_full_turn_rad](/crates/oxide-gfx/src/primitive/arc/arc_is_full_turn_rad.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
| called_by | [arc_screen_span_for](/crates/oxide-app/src/renderer_scene_canvas/arc_screen_span_for.md) |
| called_by | [draw_arc_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_arc_bucket.md) |
| called_by | [span](/crates/oxide-app/src/renderer_scene_canvas/span.md) |
