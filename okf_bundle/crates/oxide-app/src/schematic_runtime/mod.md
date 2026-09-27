---
okf_version: "0.2"
type: Module
title: schematic_runtime
description: "Local schematic runtime used by `oxide-app`."
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod
language: rust
---

# schematic_runtime

Local schematic runtime used by `oxide-app`.

## Docstring

Local schematic runtime used by `oxide-app`.

This module keeps schematic rendering, hit-test, and overlay behavior
self-contained inside the app runtime contract.

## Relationships

| Type | Target |
|------|--------|
| related | [RenderInvalidation](/crates/oxide-app/src/schematic_runtime/mod/RenderInvalidation.md) |
| related | [intersects](/crates/oxide-app/src/schematic_runtime/mod/intersects.md) |
| related | [intersects](/crates/oxide-app/src/schematic_runtime/mod/intersects.md) |
| related | [bitor](/crates/oxide-app/src/schematic_runtime/mod/bitor.md) |
| related | [bitor](/crates/oxide-app/src/schematic_runtime/mod/bitor.md) |
| related | [bitor_assign](/crates/oxide-app/src/schematic_runtime/mod/bitor_assign.md) |
| related | [bitor_assign](/crates/oxide-app/src/schematic_runtime/mod/bitor_assign.md) |
| related | [ScreenTransform](/crates/oxide-app/src/schematic_runtime/mod/ScreenTransform.md) |
| related | [world_to_screen](/crates/oxide-app/src/schematic_runtime/mod/world_to_screen.md) |
| related | [world_to_screen](/crates/oxide-app/src/schematic_runtime/mod/world_to_screen.md) |
| related | [SchematicSheetExt](/crates/oxide-app/src/schematic_runtime/mod/SchematicSheetExt.md) |
| related | [symbol_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_position.md) |
| related | [symbol_reference_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_reference_position.md) |
| related | [symbol_value_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_value_position.md) |
| related | [symbol_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_position.md) |
| related | [symbol_reference_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_reference_position.md) |
| related | [symbol_value_position](/crates/oxide-app/src/schematic_runtime/mod/symbol_value_position.md) |
| related | [SchematicRenderCache](/crates/oxide-app/src/schematic_runtime/mod/SchematicRenderCache.md) |
| related | [from_sheet](/crates/oxide-app/src/schematic_runtime/mod/from_sheet.md) |
| related | [update_from_sheet](/crates/oxide-app/src/schematic_runtime/mod/update_from_sheet.md) |
| related | [snapshot](/crates/oxide-app/src/schematic_runtime/mod/snapshot.md) |
| related | [prepared_preview](/crates/oxide-app/src/schematic_runtime/mod/prepared_preview.md) |
| related | [from_sheet](/crates/oxide-app/src/schematic_runtime/mod/from_sheet.md) |
| related | [update_from_sheet](/crates/oxide-app/src/schematic_runtime/mod/update_from_sheet.md) |
| related | [snapshot](/crates/oxide-app/src/schematic_runtime/mod/snapshot.md) |
| related | [prepared_preview](/crates/oxide-app/src/schematic_runtime/mod/prepared_preview.md) |
| related | [instance_transform](/crates/oxide-app/src/schematic_runtime/mod/instance_transform.md) |
| related | [draw_power_port_preview](/crates/oxide-app/src/schematic_runtime/mod/draw_power_port_preview.md) |
| related | [render_schematic](/crates/oxide-app/src/schematic_runtime/mod/render_schematic.md) |
| related | [render_schematic_with_renderer](/crates/oxide-app/src/schematic_runtime/mod/render_schematic_with_renderer.md) |
| related | [label_marker_polygon](/crates/oxide-app/src/schematic_runtime/mod/label_marker_polygon.md) |
| related | [renderer_id](/crates/oxide-app/src/schematic_runtime/mod/renderer_id.md) |
| related | [draw_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/mod/draw_renderer_snapshot.md) |
| related | [to_rgba](/crates/oxide-app/src/schematic_runtime/mod/to_rgba.md) |
| related | [stroke_world_mm](/crates/oxide-app/src/schematic_runtime/mod/stroke_world_mm.md) |
| related | [screen_px_to_world_mm](/crates/oxide-app/src/schematic_runtime/mod/screen_px_to_world_mm.md) |
| related | [circle_vertices](/crates/oxide-app/src/schematic_runtime/mod/circle_vertices.md) |
| related | [ItemBound](/crates/oxide-app/src/schematic_runtime/mod/ItemBound.md) |
| related | [collect_item_bounds](/crates/oxide-app/src/schematic_runtime/mod/collect_item_bounds.md) |
| related | [item_aabb](/crates/oxide-app/src/schematic_runtime/mod/item_aabb.md) |
| related | [label_color](/crates/oxide-app/src/schematic_runtime/mod/label_color.md) |
| related | [symbol_body_aabb](/crates/oxide-app/src/schematic_runtime/mod/symbol_body_aabb.md) |
| related | [text_prop_aabb](/crates/oxide-app/src/schematic_runtime/mod/text_prop_aabb.md) |
| related | [note_aabb](/crates/oxide-app/src/schematic_runtime/mod/note_aabb.md) |
| related | [label_aabb](/crates/oxide-app/src/schematic_runtime/mod/label_aabb.md) |
| related | [drawing_aabb](/crates/oxide-app/src/schematic_runtime/mod/drawing_aabb.md) |
| related | [point_to_segment_distance](/crates/oxide-app/src/schematic_runtime/mod/point_to_segment_distance.md) |
| related | [point_in_polygon](/crates/oxide-app/src/schematic_runtime/mod/point_in_polygon.md) |
| related | [stroke_px_at_zoom](/crates/oxide-app/src/schematic_runtime/mod/stroke_px_at_zoom.md) |
| related | [to_iced](/crates/oxide-app/src/schematic_runtime/mod/to_iced.md) |
| related | [focus_color](/crates/oxide-app/src/schematic_runtime/mod/focus_color.md) |
| related | [aabb_overlaps](/crates/oxide-app/src/schematic_runtime/mod/aabb_overlaps.md) |
| related | [line_visible](/crates/oxide-app/src/schematic_runtime/mod/line_visible.md) |
| related | [rect_visible](/crates/oxide-app/src/schematic_runtime/mod/rect_visible.md) |
| related | [point_visible](/crates/oxide-app/src/schematic_runtime/mod/point_visible.md) |
| related | [resolve_stroke_color](/crates/oxide-app/src/schematic_runtime/mod/resolve_stroke_color.md) |
| related | [fill_color_for](/crates/oxide-app/src/schematic_runtime/mod/fill_color_for.md) |
| related | [arc_sweeps_through_mid](/crates/oxide-app/src/schematic_runtime/mod/arc_sweeps_through_mid.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
