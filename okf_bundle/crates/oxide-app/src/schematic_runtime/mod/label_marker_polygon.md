---
okf_version: "0.2"
type: Function
title: label_marker_polygon
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/label_marker_polygon
language: rust
---

# label_marker_polygon

## Signature

```rust
fn label_marker_polygon(
    label: &Label,
    stroke_color: Color,
    fill_color: [f32; 4],
    transform: &ScreenTransform,
) -> PolygonInput
```

## Source
Lines 299–340 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [to_rgba](/crates/oxide-app/src/schematic_runtime/mod/to_rgba.md) |
| calls | [stroke_world_mm](/crates/oxide-app/src/schematic_runtime/mod/stroke_world_mm.md) |
| called_by | [draw_label_preview](/crates/oxide-app/src/schematic_runtime/label/draw_label_preview.md) |
| called_by | [build_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/snapshot/build_renderer_snapshot.md) |
