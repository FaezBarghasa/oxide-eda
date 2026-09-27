---
okf_version: "0.2"
type: Function
title: draw_text_bucket
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/draw_text_bucket
language: rust
---

# draw_text_bucket

## Signature

```rust
fn draw_text_bucket(
    frame: &mut canvas::Frame,
    texts: &[TextItem],
    world_to_screen: F,
    options: SceneDrawOptions,
)
```

## Type Parameters

- `F`

## Source
Lines 343–381 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [color_from_rgba](/crates/oxide-app/src/renderer_scene_canvas/color_from_rgba.md) |
| calls | [to_text_h_align](/crates/oxide-app/src/renderer_scene_canvas/to_text_h_align.md) |
| calls | [to_text_v_align](/crates/oxide-app/src/renderer_scene_canvas/to_text_v_align.md) |
| calls | [translate](/crates/oxide-app/src/app/view/translate/translate.md) |
| called_by | [draw_scene_with_world_to_screen](/crates/oxide-app/src/renderer_scene_canvas/draw_scene_with_world_to_screen.md) |
