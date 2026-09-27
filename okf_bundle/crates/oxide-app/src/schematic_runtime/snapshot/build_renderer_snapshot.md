---
okf_version: "0.2"
type: Function
title: build_renderer_snapshot
resource: crates/oxide-app/src/schematic_runtime/snapshot.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/snapshot/build_renderer_snapshot
language: rust
---

# build_renderer_snapshot

## Signature

```rust
pub(super) fn build_renderer_snapshot(
    snapshot: &SchematicRenderSnapshot,
    transform: &ScreenTransform,
    colors: &CanvasColors,
    bounds: Rectangle,
    focus_set: Option<&HashSet<uuid::Uuid>>,
    wire_color_overrides: Option<&HashMap<uuid::Uuid, ThemeColor>>,
) -> RendererSnapshot
```

## Visibility

- `pub(super)`

## Source
Lines 3–490 in `crates/oxide-app/src/schematic_runtime/snapshot.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snapshot](/crates/oxide-app/src/schematic_runtime/snapshot.md) |
| calls | [line_visible](/crates/oxide-app/src/schematic_runtime/mod/line_visible.md) |
| calls | [focus_color](/crates/oxide-app/src/schematic_runtime/mod/focus_color.md) |
| calls | [renderer_id](/crates/oxide-app/src/schematic_runtime/mod/renderer_id.md) |
| calls | [point_visible](/crates/oxide-app/src/schematic_runtime/mod/point_visible.md) |
| calls | [symbol_body_aabb](/crates/oxide-app/src/schematic_runtime/mod/symbol_body_aabb.md) |
| calls | [rect_visible](/crates/oxide-app/src/schematic_runtime/mod/rect_visible.md) |
| calls | [drawing_aabb](/crates/oxide-app/src/schematic_runtime/mod/drawing_aabb.md) |
| calls | [resolve_stroke_color](/crates/oxide-app/src/schematic_runtime/mod/resolve_stroke_color.md) |
| calls | [fill_color_for](/crates/oxide-app/src/schematic_runtime/mod/fill_color_for.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
| calls | [arc_sweeps_through_mid](/crates/oxide-app/src/schematic_runtime/mod/arc_sweeps_through_mid.md) |
| calls | [label_color](/crates/oxide-app/src/schematic_runtime/mod/label_color.md) |
| calls | [label_marker_polygon](/crates/oxide-app/src/schematic_runtime/mod/label_marker_polygon.md) |
| called_by | [render_schematic_with_renderer](/crates/oxide-app/src/schematic_runtime/mod/render_schematic_with_renderer.md) |
