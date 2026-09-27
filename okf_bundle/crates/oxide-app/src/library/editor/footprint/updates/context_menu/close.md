---
okf_version: "0.2"
type: Function
title: close
resource: crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/context_menu/close
language: rust
---

# close

## Signature

```rust
fn close(editor: &mut crate::app::FootprintEditorState)
```

## Source
Lines 73–75 in `crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/updates/context_menu.md) |
| called_by | [dispatch_window_message](/crates/oxide-app/src/app/dispatch/mod/dispatch_window_message.md) |
| called_by | [close_main_window_now](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/close_main_window_now.md) |
| called_by | [close_detached_modal](/crates/oxide-app/src/app/handlers/erc/modals/close_detached_modal.md) |
| called_by | [draw_net_color_pen](/crates/oxide-app/src/canvas/draw/overlay/draw_net_color_pen.md) |
| called_by | [draw_place_pad_ghost](/crates/oxide-app/src/library/editor/footprint/canvas/draw/ghosts/draw_place_pad_ghost.md) |
| called_by | [draw_grid_dots](/crates/oxide-app/src/library/editor/footprint/canvas/draw/grid/draw_grid_dots.md) |
| called_by | [draw_select_cursor_mark](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_select_cursor_mark.md) |
| called_by | [draw_pad](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pad.md) |
| called_by | [draw_filled_closed_loops](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/draw_filled_closed_loops.md) |
| called_by | [draw](/crates/oxide-app/src/library/editor/footprint/preview3d/draw.md) |
| called_by | [fill_quad](/crates/oxide-app/src/library/editor/footprint/preview3d/fill_quad.md) |
| called_by | [stroke_quad](/crates/oxide-app/src/library/editor/footprint/preview3d/stroke_quad.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/apply.md) |
| called_by | [draw](/crates/oxide-app/src/panels/element_properties/drawing_preview/draw.md) |
| called_by | [draw](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/draw.md) |
| called_by | [pad_stack_preview](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/pad_stack_preview.md) |
| called_by | [draw_polygons](/crates/oxide-app/src/pcb_canvas/draw_polygons.md) |
| called_by | [draw_polygon_bucket](/crates/oxide-app/src/renderer_scene_canvas/draw_polygon_bucket.md) |
