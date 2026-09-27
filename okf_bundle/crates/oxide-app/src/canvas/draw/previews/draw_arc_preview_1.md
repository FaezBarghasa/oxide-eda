---
okf_version: "0.2"
type: Function
title: draw_arc_preview
description: Arc-in-progress preview.
resource: crates/oxide-app/src/canvas/draw/previews.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/canvas/draw/previews/draw_arc_preview_1
language: rust
---

# draw_arc_preview

Arc-in-progress preview.

## Signature

```rust
pub(in crate::canvas) fn draw_arc_preview(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        cursor_pos: iced::Point,
    )
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Arc-in-progress preview.

## Source
Lines 106–204 in `crates/oxide-app/src/canvas/draw/previews.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [previews](/crates/oxide-app/src/canvas/draw/previews.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
| calls | [arc_sweeps_through_mid](/crates/oxide-app/src/schematic_runtime/mod/arc_sweeps_through_mid.md) |
| calls | [arc_screen_span_for](/crates/oxide-app/src/renderer_scene_canvas/arc_screen_span_for.md) |
