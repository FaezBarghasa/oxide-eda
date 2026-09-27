---
okf_version: "0.2"
type: Function
title: rectangle
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle
language: rust
---

# rectangle

## Signature

```rust
fn rectangle(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Source
Lines 340–432 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [draw](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [draw_background](/crates/oxide-app/src/canvas/draw/background/draw_background.md) |
| called_by | [draw_select_rect](/crates/oxide-app/src/canvas/draw/drag/draw_select_rect.md) |
| called_by | [draw_tool_chip](/crates/oxide-app/src/canvas/draw/overlay/draw_tool_chip.md) |
| called_by | [draw_shape_preview](/crates/oxide-app/src/canvas/draw/previews/draw_shape_preview.md) |
| called_by | [draw_place_pad_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts/draw_place_pad_ghost.md) |
| called_by | [draw_rubber_band](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_rubber_band.md) |
| called_by | [draw_sketch_reticle](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_reticle.md) |
| called_by | [draw_pad](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pad.md) |
| called_by | [draw_pads_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pads_tool_preview.md) |
| called_by | [draw_array_badges](/crates/oxide-app/src/library/editor/footprint/canvas/draw/scene/draw_array_badges.md) |
| called_by | [draw_courtyard](/crates/oxide-app/src/library/editor/footprint/canvas/draw/scene/draw_courtyard.md) |
| called_by | [draw_silk_graphics](/crates/oxide-app/src/library/editor/footprint/canvas/draw/silk/draw_silk_graphics.md) |
| called_by | [draw_dim_pill_styled](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_dim_pill_styled.md) |
| called_by | [draw_sketch_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_sketch_tool_preview.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/apply.md) |
| called_by | [draw_box_select_overlay](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_box_select_overlay.md) |
| called_by | [draw_rect_preview](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_rect_preview.md) |
| called_by | [draw_resize_handles](/crates/oxide-app/src/library/editor/symbol/canvas/draw/scene/draw_resize_handles.md) |
| called_by | [draw](/crates/oxide-app/src/panels/element_properties/drawing_preview/draw.md) |
