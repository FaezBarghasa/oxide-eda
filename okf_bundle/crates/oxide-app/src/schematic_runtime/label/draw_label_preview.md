---
okf_version: "0.2"
type: Function
title: draw_label_preview
resource: crates/oxide-app/src/schematic_runtime/label.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/schematic_runtime/label/draw_label_preview
language: rust
---

# draw_label_preview

## Signature

```rust
pub fn draw_label_preview(
    frame: &mut canvas::Frame,
    label: &Label,
    transform: &ScreenTransform,
    stroke_color: Color,
    fill_color: Color,
)
```

## Visibility

- `pub`

## Source
Lines 3–75 in `crates/oxide-app/src/schematic_runtime/label.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [label](/crates/oxide-app/src/schematic_runtime/label.md) |
| calls | [label_marker_polygon](/crates/oxide-app/src/schematic_runtime/mod/label_marker_polygon.md) |
| calls | [draw_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/mod/draw_renderer_snapshot.md) |
| calls | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| called_by | [draw_ghost_label](/crates/oxide-app/src/canvas/draw/ghosts/draw_ghost_label.md) |
