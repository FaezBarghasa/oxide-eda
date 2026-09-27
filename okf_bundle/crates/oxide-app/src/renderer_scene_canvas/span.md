---
okf_version: "0.2"
type: Function
title: span
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/span
language: rust
---

# span

## Signature

```rust
fn span(start: f32, end: f32, y_up: bool) -> (f32, f32)
```

## Source
Lines 434–439 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [arc_screen_span](/crates/oxide-app/src/renderer_scene_canvas/arc_screen_span.md) |
| called_by | [a_schematic_arc_bulges_toward_the_point_the_user_clicked](/crates/oxide-app/src/renderer_scene_canvas/a_schematic_arc_bulges_toward_the_point_the_user_clicked.md) |
| called_by | [a_symbol_editor_arc_reflects_because_its_canvas_flips_y](/crates/oxide-app/src/renderer_scene_canvas/a_symbol_editor_arc_reflects_because_its_canvas_flips_y.md) |
| called_by | [a_wrapped_arc_sweeps_the_short_way_in_both_frames](/crates/oxide-app/src/renderer_scene_canvas/a_wrapped_arc_sweeps_the_short_way_in_both_frames.md) |
| called_by | [both_frames_sweep_the_same_magnitude](/crates/oxide-app/src/renderer_scene_canvas/both_frames_sweep_the_same_magnitude.md) |
| called_by | [parse](/crates/oxide-erc-dsl/src/parser/parse.md) |
