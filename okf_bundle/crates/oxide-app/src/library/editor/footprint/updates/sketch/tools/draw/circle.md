---
okf_version: "0.2"
type: Function
title: circle
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle
language: rust
---

# circle

## Signature

```rust
fn circle(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Source
Lines 138–188 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [draw](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [draw_move_guides](/crates/oxide-app/src/canvas/draw/drag/draw_move_guides.md) |
| called_by | [draw_arc_preview](/crates/oxide-app/src/canvas/draw/previews/draw_arc_preview.md) |
| called_by | [draw_lasso_preview](/crates/oxide-app/src/canvas/draw/previews/draw_lasso_preview.md) |
| called_by | [draw_shape_preview](/crates/oxide-app/src/canvas/draw/previews/draw_shape_preview.md) |
| called_by | [draw_grid](/crates/oxide-app/src/canvas/grid/draw_grid.md) |
| called_by | [draw_place_pad_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts/draw_place_pad_ghost.md) |
| called_by | [draw_place_via_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts/draw_place_via_ghost.md) |
| called_by | [draw_lasso_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_lasso_ghost.md) |
| called_by | [draw_touching_line_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_touching_line_ghost.md) |
| called_by | [draw_pads_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pads_tool_preview.md) |
| called_by | [draw_silk_graphics](/crates/oxide-app/src/library/editor/footprint/canvas/draw/silk/draw_silk_graphics.md) |
| called_by | [draw_sketch_overlay](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/draw_sketch_overlay.md) |
| called_by | [draw_sketch_snap_glyph](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/snap/draw_sketch_snap_glyph.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/apply.md) |
| called_by | [draw_grid](/crates/oxide-app/src/library/editor/symbol/canvas/draw/background/draw_grid.md) |
| called_by | [draw_origin_marker](/crates/oxide-app/src/library/editor/symbol/canvas/draw/background/draw_origin_marker.md) |
| called_by | [draw_arc_preview](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_arc_preview.md) |
| called_by | [draw_circle_preview](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_circle_preview.md) |
| called_by | [draw_line_preview](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_line_preview.md) |
| called_by | [draw_polygon_preview](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_polygon_preview.md) |
| called_by | [draw_rect_preview](/crates/oxide-app/src/library/editor/symbol/canvas/draw/overlays/draw_rect_preview.md) |
| called_by | [draw](/crates/oxide-app/src/panels/element_properties/drawing_preview/draw.md) |
| called_by | [draw](/crates/oxide-app/src/panels/telecom/canvas/draw.md) |
| called_by | [draw_circles](/crates/oxide-app/src/pcb_canvas/draw_circles.md) |
| called_by | [draw_arc_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_arc_bucket.md) |
| called_by | [draw_circle_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_circle_bucket.md) |
| called_by | [draw](/crates/oxide-widgets/src/symbol_preview/draw.md) |
